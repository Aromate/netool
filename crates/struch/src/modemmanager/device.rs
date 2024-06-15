use std::{fmt::Display, io};

use super::{
    bearer::BearerInfo,
    list::ModemIDList,
    modem::{ModemInfo, PortList},
};
use crate::iproute2::DeviceList;

#[derive(Debug, Default)]
pub struct ModemDevice {
    pub connected: bool,
    pub device_name: String,
    pub hardware_name: String,
    pub device_id: u32, //net ifindex
    pub net_ip: String,
    pub sim_ip: String,
    pub modem: ModemInfo,
    pub bearer: Vec<BearerInfo>,
}

impl ModemDevice {
    pub fn new(modem_id: impl Display) -> io::Result<Self> {
        let modem_info = ModemInfo::new(modem_id)?;
        let mut modem_device = ModemDevice::default();
        let ports = PortList::from_vec_string(&modem_info.modem.generic.ports);
        let device_name = ports.get_net_name();
        modem_device.hardware_name = ports.get_dev_name();
        match DeviceList::new()?.find_by_name(&device_name) {
            Some(device) => {
                modem_device.device_id = device.ifindex;
                modem_device.net_ip = device.get_ip_addr();
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Device {} not found", modem_device.device_name),
                ));
            }
        }
        modem_device.device_name = modem_info.net_device_name();
        modem_device.connected = modem_info.modem.generic.state == "connected";
        for bearer in modem_info.modem.generic.bearers.iter() {
            let bearer = BearerInfo::new(bearer)?;
            modem_device.sim_ip = bearer.bearer.ipv4_config.address.to_string();
            modem_device.bearer.push(bearer);
        }
        modem_device.modem = modem_info;
        Ok(modem_device)
    }
}

#[derive(Debug, Default)]
pub struct ModemDeviceList(Vec<ModemDevice>);

impl ModemDeviceList {
    pub fn new() -> io::Result<Self> {
        let mut modem_device_list = ModemDeviceList::default();
        ModemIDList::new()?
            .modem_id_list
            .iter()
            .for_each(|id| modem_device_list.0.push(ModemDevice::new(id).unwrap()));
        Ok(modem_device_list)
    }

    pub fn get_modem_device_list(&self) -> &Vec<ModemDevice> {
        &self.0
    }

    pub fn find_by_id(&self, id: u32) -> Option<&ModemDevice> {
        self.0.iter().find(|d| d.device_id == id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&ModemDevice> {
        self.0.iter().find(|d| d.device_name == name)
    }
}
