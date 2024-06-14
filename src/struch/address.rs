use std::{fmt::Display, io, str::FromStr};

use cmd_lib::run_fun;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressList(Vec<Address>);

impl Display for AddressList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl FromStr for AddressList {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl AddressList {
    #[allow(dead_code)]
    pub fn new() -> io::Result<Self> {
        run_fun!(
            ip -j address show
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn get_device_addr(device: impl Display) -> io::Result<Self> {
        let device = device.to_string();
        run_fun!(
            ip -j address show dev $device
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn get_address_list(&self) -> &Vec<Address> {
        &self.0
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub ifindex: i64,
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
        let ip_list = AddressList::new().unwrap();
        println!("{}", ip_list);
    }
}
