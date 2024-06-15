use std::{fmt::Display, io, str::FromStr};

use cmd_lib::run_fun;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModemInfo {
    pub modem: Modem,
}

impl FromStr for ModemInfo {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for ModemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl ModemInfo {
    pub fn new(modem_id: impl Display) -> io::Result<ModemInfo> {
        run_fun!(
            mmcli -m $modem_id -J
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn net_device_name(&self) -> String {
        let port_list = PortList::from_vec_string(self.modem.generic.ports.as_ref());
        port_list.get_net_name()
    }

    pub fn disconnect(&self) -> io::Result<()> {
        let dbus_path = self.modem.dbus_path.clone();
        run_fun!(
            mmcli -m $dbus_path --simple-disconnect
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .map(|_| ())
    }

    pub fn connect(&self, apn: &str) -> io::Result<()> {
        let dbus_path = self.modem.dbus_path.clone();
        run_fun!(
            mmcli -m $dbus_path --simple-connect="apn=$apn"
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .map(|_| ())
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        let dbus_path = self.modem.dbus_path.clone();
        self.modem = Self::new(dbus_path)?.modem;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modem {
    #[serde(rename = "3gpp")]
    pub n3gpp: N3gpp,
    pub cdma: Cdma,
    #[serde(rename = "dbus-path")]
    pub dbus_path: String,
    pub generic: Generic,
}

impl FromStr for Modem {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N3gpp {
    #[serde(rename = "5gnr")]
    pub n5gnr: N5gnr,
    #[serde(rename = "enabled-locks")]
    pub enabled_locks: Vec<String>,
    pub eps: Eps,
    pub imei: String,
    #[serde(rename = "operator-code")]
    pub operator_code: String,
    #[serde(rename = "operator-name")]
    pub operator_name: String,
    #[serde(rename = "packet-service-state")]
    pub packet_service_state: String,
    pub pco: String,
    #[serde(rename = "registration-state")]
    pub registration_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N5gnr {
    #[serde(rename = "registration-settings")]
    pub registration_settings: RegistrationSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationSettings {
    #[serde(rename = "drx-cycle")]
    pub drx_cycle: String,
    #[serde(rename = "mico-mode")]
    pub mico_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eps {
    #[serde(rename = "initial-bearer")]
    pub initial_bearer: InitialBearer,
    #[serde(rename = "ue-mode-operation")]
    pub ue_mode_operation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialBearer {
    #[serde(rename = "dbus-path")]
    pub dbus_path: String,
    pub settings: Settings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub apn: String,
    #[serde(rename = "ip-type")]
    pub ip_type: String,
    pub password: String,
    pub user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cdma {
    #[serde(rename = "activation-state")]
    pub activation_state: String,
    #[serde(rename = "cdma1x-registration-state")]
    pub cdma1x_registration_state: String,
    pub esn: String,
    #[serde(rename = "evdo-registration-state")]
    pub evdo_registration_state: String,
    pub meid: String,
    pub nid: String,
    pub sid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Generic {
    #[serde(rename = "access-technologies")]
    pub access_technologies: Vec<String>,
    pub bearers: Vec<String>,
    #[serde(rename = "carrier-configuration")]
    pub carrier_configuration: String,
    #[serde(rename = "carrier-configuration-revision")]
    pub carrier_configuration_revision: String,
    #[serde(rename = "current-bands")]
    pub current_bands: Vec<Value>,
    #[serde(rename = "current-capabilities")]
    pub current_capabilities: Vec<String>,
    #[serde(rename = "current-modes")]
    pub current_modes: String,
    pub device: String,
    #[serde(rename = "device-identifier")]
    pub device_identifier: String,
    pub drivers: Vec<String>,
    #[serde(rename = "equipment-identifier")]
    pub equipment_identifier: String,
    #[serde(rename = "hardware-revision")]
    pub hardware_revision: String,
    pub manufacturer: String,
    pub model: String,
    #[serde(rename = "own-numbers")]
    pub own_numbers: Vec<Value>,
    pub plugin: String,
    pub ports: Vec<String>,
    #[serde(rename = "power-state")]
    pub power_state: String,
    #[serde(rename = "primary-port")]
    pub primary_port: String,
    #[serde(rename = "primary-sim-slot")]
    pub primary_sim_slot: String,
    pub revision: String,
    #[serde(rename = "signal-quality")]
    pub signal_quality: SignalQuality,
    pub sim: String,
    #[serde(rename = "sim-slots")]
    pub sim_slots: Vec<Value>,
    pub state: String,
    #[serde(rename = "state-failed-reason")]
    pub state_failed_reason: String,
    #[serde(rename = "supported-bands")]
    pub supported_bands: Vec<Value>,
    #[serde(rename = "supported-capabilities")]
    pub supported_capabilities: Vec<String>,
    #[serde(rename = "supported-ip-families")]
    pub supported_ip_families: Vec<String>,
    #[serde(rename = "supported-modes")]
    pub supported_modes: Vec<String>,
    #[serde(rename = "unlock-required")]
    pub unlock_required: String,
    #[serde(rename = "unlock-retries")]
    pub unlock_retries: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalQuality {
    pub recent: String,
    pub value: String,
}

pub struct PortList {
    pub ports: Vec<Port>,
}

impl PortList {
    pub fn from_vec_string(ports: &[String]) -> PortList {
        let ports = ports
            .iter()
            .map(|port| Port::from_str(port).unwrap())
            .collect();
        PortList { ports }
    }

    pub fn get_net_name(&self) -> String {
        self.ports
            .iter()
            .find(|port| port.device_type == DeviceType::Net)
            .map(|port| port.name.clone())
            .unwrap_or_else(|| "".to_string())
    }

    pub fn get_dev_name(&self) -> String {
        self.ports
            .iter()
            .find(|port| port.device_type == DeviceType::Mbim)
            .map(|port| port.name.clone())
            .unwrap_or_else(|| "".to_string())
    }
}

pub struct Port {
    pub name: String,
    pub device_type: DeviceType,
}

#[derive(Debug, PartialEq)]
pub enum DeviceType {
    Net,
    Mbim,
    Unknown,
}

impl FromStr for DeviceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(net)" => Ok(DeviceType::Net),
            "(mbim)" => Ok(DeviceType::Mbim),
            _ => Ok(DeviceType::Unknown),
        }
    }
}

impl FromStr for Port {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let (name, device_type) = (iter.next(), iter.next());
        match (name, device_type) {
            (Some(name), Some(device_type)) => Ok(Port {
                name: name.to_string(),
                device_type: DeviceType::from_str(device_type)?,
            }),
            _ => Err("Invalid port string".to_string()),
        }
    }
}
