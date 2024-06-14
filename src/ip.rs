use std::{fmt::Display, io};

use cmd_lib::run_cmd;

use crate::struch::{address::AddressList, bearer::BearerInfo};

pub fn set_static_ip(bearer_info: &BearerInfo) -> io::Result<()> {
    if bearer_info.bearer.status.connected != "yes" {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The bearer is not connected",
        ));
    }
    if bearer_info.bearer.ipv4_config.method != "static" {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The bearer is not using static ip",
        ));
    }

    let modem_dev = bearer_info.bearer.status.interface.clone();
    if modem_dev.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The modem device is empty",
        ));
    }

    let ip = bearer_info.bearer.ipv4_config.address.clone();
    if ip.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The ip address is empty",
        ));
    }
    run_cmd!(
        sudo ip addr add $ip/32 dev $modem_dev
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

pub fn set_route(bearer_info: &BearerInfo) -> io::Result<()> {
    if bearer_info.bearer.status.connected != "yes" {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The bearer is not connected",
        ));
    }
    if bearer_info.bearer.ipv4_config.method != "static" {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The bearer is not using static ip",
        ));
    }

    let modem_dev = bearer_info.bearer.status.interface.clone();
    if modem_dev.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The modem device is empty",
        ));
    }

    let gateway = bearer_info.bearer.ipv4_config.gateway.clone();
    if gateway.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "The gateway is empty"));
    }
    run_cmd!(
        sudo ip route add default via $gateway dev $modem_dev metric 200
    ) // set the metric to 200 to make sure the route is not the first choice
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

pub fn get_all_address() -> io::Result<AddressList> {
    AddressList::new()
}

pub fn get_device_address(device: impl Display) -> io::Result<AddressList> {
    AddressList::get_device_addr(device)
}

pub fn get_modem_real_ip(bearer_info: &BearerInfo) -> io::Result<AddressList> {
    let modem_dev = bearer_info.bearer.status.interface.clone();
    if modem_dev.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "The modem device is empty",
        ));
    }
    AddressList::get_device_addr(modem_dev)
}

pub fn get_modem_apn_ip(bearer_info: &BearerInfo) -> io::Result<String> {
    let ip = bearer_info.bearer.ipv4_config.address.clone();
    if ip.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "The ip address is empty",
        ))
    } else {
        Ok(ip)
    }
}
