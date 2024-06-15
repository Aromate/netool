use std::{fmt::Display, io, str::FromStr};

use cmd_lib::run_fun;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceList(Vec<Device>);

impl Display for DeviceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl FromStr for DeviceList {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl DeviceList {
    #[allow(dead_code)]
    pub fn new() -> io::Result<Self> {
        run_fun!(
            ip -j address show
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn from_device(device_name: &str) -> io::Result<Self> {
        run_fun!(
            ip -j address show dev $device_name
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn get_address_list(&self) -> &Vec<Device> {
        &self.0
    }

    pub fn find_by_id(&self, id: u32) -> Option<&Device> {
        self.0.iter().find(|d| d.ifindex == id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Device> {
        self.0.iter().find(|d| d.ifname == name)
    }

    pub fn fist(&self) -> io::Result<&Device> {
        self.0
            .first()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No device found"))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub ifindex: u32,
    pub ifname: String,
    pub flags: Vec<String>,
    pub mtu: i64,
    pub qdisc: String,
    pub operstate: String,
    pub group: String,
    pub txqlen: i64,
    #[serde(rename = "link_type")]
    pub link_type: String,
    pub address: Option<String>,
    pub broadcast: Option<String>,
    #[serde(rename = "addr_info")]
    pub addr_info: Vec<AddrInfo>,
}

impl FromStr for Device {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Device {
    pub fn new(device_name: &str) -> io::Result<Self> {
        let device_list = DeviceList::from_device(device_name)?;
        let device = device_list.fist()?.to_owned();
        Ok(device)
    }

    pub fn add_ip(&self, ip_addr: &str) -> io::Result<()> {
        let ifname = self.ifname.clone();
        run_fun!(
            ip addr add $ip_addr dev $ifname
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .map(|_| ())
    }

    pub fn del_route(&self) -> io::Result<()> {
        let ifname = self.ifname.clone();
        run_fun!(
            ip route del default dev $ifname metric 200
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .map(|_| ())
    }

    pub fn set_up(&self) -> io::Result<()> {
        let ifname = self.ifname.clone();
        run_fun!(
            ip link set $ifname up
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .map(|_| ())
    }

    pub fn set_down(&self) -> io::Result<()> {
        let ifname = self.ifname.clone();
        run_fun!(
            ip link set $ifname down
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .map(|_| ())
    }

    pub fn flush(&self) -> io::Result<()> {
        let ifname = self.ifname.clone();
        run_fun!(
            ip addr flush dev $ifname
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .map(|_| ())
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        let ifname = self.ifname.clone();
        self.clone_from(&Self::new(&ifname)?);
        Ok(())
    }

    pub fn get_ip_addr(&self) -> String {
        let addrs = self
            .addr_info
            .iter()
            .map(|addr| addr.to_string())
            .collect::<Vec<String>>();
        addrs.join(", ")
    }

    pub fn set_ip_route(&self) -> io::Result<()> {
        let ifname = self.ifname.clone();
        run_fun!(
            ip route add default dev $ifname metric 200
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .map(|_| ())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddrInfo {
    pub family: String,
    pub local: String,
    pub prefixlen: i64,
    pub broadcast: Option<String>,
    pub scope: String,
    pub label: Option<String>,
    #[serde(rename = "valid_life_time")]
    pub valid_life_time: i64,
    #[serde(rename = "preferred_life_time")]
    pub preferred_life_time: i64,
    pub dynamic: Option<bool>,
    pub noprefixroute: Option<bool>,
    pub protocol: Option<String>,
}

impl Display for AddrInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.local, self.prefixlen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_list() {
        let ip_list = DeviceList::new().unwrap();
        println!("{}", ip_list);
    }
}
