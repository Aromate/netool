use std::{collections::HashSet, fmt::Display, fs, io, str::FromStr};

use serde_derive::{Deserialize, Serialize};

static RESOLV_CONF: &str = "/etc/resolv.conf";

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolv {
    pub first_record: usize,
    pub nameserver: HashSet<String>,
    pub add_nameserver: HashSet<String>,
}

impl FromStr for Resolv {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for Resolv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Resolv {
    pub fn new() -> io::Result<Self> {
        let file = std::fs::read_to_string(RESOLV_CONF)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut resolv = Self {
            first_record: 0,
            ..Default::default()
        };
        for (index, line) in file.lines().enumerate() {
            if line.starts_with("nameserver") {
                if resolv.first_record == 0 {
                    resolv.first_record = index;
                }
                let ns = line.split_whitespace().last().unwrap_or("").to_string();
                resolv.nameserver.insert(ns);
            }
        }
        Ok(resolv)
    }

    pub fn add_resolv(&mut self, ip_addr: impl Display) {
        let ip_addr = ip_addr.to_string();
        if !self.nameserver.contains(&ip_addr) {
            self.add_nameserver.insert(ip_addr);
        }
    }

    pub fn update_resolv(&mut self) -> io::Result<()> {
        let file =
            fs::read_to_string(RESOLV_CONF).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut lines = file.lines().map(|s| s.to_string()).collect::<Vec<String>>();
        if self.first_record == 0 {
            self.first_record = lines.len();
        }
        for ns in &self.add_nameserver {
            lines.insert(self.first_record, format!("nameserver {}", ns));
        }
        let content = lines.join("\n");
        fs::write(RESOLV_CONF, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolv() {
        let resolv = Resolv::new().unwrap();
        println!("{:?}", resolv);
    }

    #[test]
    fn test_add_resolv() {
        let mut resolv = Resolv::new().unwrap();
        resolv.add_resolv("8.8.8.8");
        resolv.add_resolv("1.1.11.1");
        resolv.add_resolv("1.1.11.1");
        resolv.add_resolv("1.1.11.1");
        resolv.add_resolv("1.1.81.1");
        resolv.add_resolv("1.1.41.1");
        resolv.add_resolv("1.1.11.1");
        resolv.add_resolv("1.1.11.2");
        println!("{:?}", resolv);
        resolv.update_resolv().unwrap();
    }
}
