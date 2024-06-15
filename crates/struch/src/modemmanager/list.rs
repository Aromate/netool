use std::{fmt::Display, io, str::FromStr};

use cmd_lib::run_fun;
use serde_derive::{Deserialize, Serialize};

use super::{bearer::BearerInfo, modem::ModemInfo};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModemIDList {
    #[serde(rename = "modem-list")]
    pub modem_id_list: Vec<String>,
}

impl FromStr for ModemIDList {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for ModemIDList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl ModemIDList {
    #[rustfmt::skip]
    pub fn new() -> io::Result<ModemIDList> {
        run_fun!(mmcli -L -J) // can not format
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn get_modem_list(&self) -> io::Result<Vec<ModemInfo>> {
        let mut modem_list = Vec::new();
        for id in &self.modem_id_list {
            let modem = ModemInfo::new(id)?;
            modem_list.push(modem);
        }
        Ok(modem_list)
    }

    pub fn get_bearer_list(&self) -> io::Result<Vec<BearerInfo>> {
        let mut bearer_list = Vec::new();
        for id in &self.modem_id_list {
            let bearer = BearerInfo::new(id)?;
            bearer_list.push(bearer);
        }
        Ok(bearer_list)
    }
}
