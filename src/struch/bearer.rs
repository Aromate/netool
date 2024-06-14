use std::{fmt::Display, str::FromStr};

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BearerInfo {
    pub bearer: Bearer,
}

impl FromStr for BearerInfo {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for BearerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bearer {
    #[serde(rename = "dbus-path")]
    pub dbus_path: String,
    #[serde(rename = "ipv4-config")]
    pub ipv4_config: Ipv4Config,
    #[serde(rename = "ipv6-config")]
    pub ipv6_config: Ipv6Config,
    pub properties: Properties,
    pub stats: Stats,
    pub status: Status,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ipv4Config {
    pub address: String,
    pub dns: Vec<String>,
    pub gateway: String,
    pub method: String,
    pub mtu: String,
    pub prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ipv6Config {
    pub address: String,
    pub dns: Vec<Value>,
    pub gateway: String,
    pub method: String,
    pub mtu: String,
    pub prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "access-type-preference")]
    pub access_type_preference: String,
    #[serde(rename = "allowed-auth")]
    pub allowed_auth: Vec<Value>,
    pub apn: String,
    #[serde(rename = "apn-type")]
    pub apn_type: String,
    #[serde(rename = "ip-type")]
    pub ip_type: String,
    pub password: String,
    #[serde(rename = "profile-id")]
    pub profile_id: String,
    #[serde(rename = "rm-protocol")]
    pub rm_protocol: String,
    pub roaming: String,
    #[serde(rename = "roaming-allowance")]
    pub roaming_allowance: String,
    pub user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub attempts: String,
    #[serde(rename = "bytes-rx")]
    pub bytes_rx: String,
    #[serde(rename = "bytes-tx")]
    pub bytes_tx: String,
    #[serde(rename = "downlink-speed")]
    pub downlink_speed: String,
    pub duration: String,
    #[serde(rename = "failed-attempts")]
    pub failed_attempts: String,
    #[serde(rename = "start-date")]
    pub start_date: String,
    #[serde(rename = "total-bytes-rx")]
    pub total_bytes_rx: String,
    #[serde(rename = "total-bytes-tx")]
    pub total_bytes_tx: String,
    #[serde(rename = "total-duration")]
    pub total_duration: String,
    #[serde(rename = "uplink-speed")]
    pub uplink_speed: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub connected: String,
    #[serde(rename = "connection-error")]
    pub connection_error: ConnectionError,
    pub interface: String,
    #[serde(rename = "ip-timeout")]
    pub ip_timeout: String,
    pub multiplexed: String,
    #[serde(rename = "profile-id")]
    pub profile_id: String,
    pub suspended: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionError {
    pub message: String,
    pub name: String,
}
