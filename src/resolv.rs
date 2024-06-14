use std::{
    collections::HashSet,
    fmt::Display,
    io::{self, Write},
    str::FromStr,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resolv {
    pub nameserver: HashSet<String>,
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
        let file = std::fs::read_to_string("/etc/resolv.conf")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut nameserver = Vec::new();
        for line in file.lines() {
            if line.starts_with("nameserver") {
                let ns = line.split_whitespace().last().unwrap().to_string();
                nameserver.push(ns);
            }
        }
        let nameserver = nameserver.into_iter().collect();
        Ok(Self { nameserver })
    }

    pub fn add_resolv(&mut self, ip_addr: impl Display) {
        self.nameserver.insert(ip_addr.to_string());
    }

    pub fn update_resolv(&mut self) -> io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("/etc/resolv.conf")?;
        for ns in &self.nameserver {
            writeln!(file, "nameserver {}", ns)?;
        }
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
        println!("{:?}", resolv);
        resolv.update_resolv().unwrap();
    }
}
