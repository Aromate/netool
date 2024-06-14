use std::io;

use cmd_lib::run_fun;

use crate::struch::{bearer::BearerInfo, list::ModemList, modem::ModemInfo};

pub fn modem_info(modem_id: Option<u32>) -> io::Result<ModemInfo> {
    let modem = modem_id.unwrap_or(0);
    run_fun!(
        mmcli -m $modem -J
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    .parse()
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn bearer_info(bearer_id: Option<u32>) -> io::Result<BearerInfo> {
    let bearer = bearer_id.unwrap_or(0);
    run_fun!(
        mmcli -b $bearer -J
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    .parse()
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn find_modem() -> io::Result<ModemList> {
    run_fun!(mmcli - L - J)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn connect(modem_id: Option<u32>, apn: Option<String>) -> io::Result<()> {
    let modem = modem_id.unwrap_or(0);
    let apn = apn.unwrap_or("cmnet".to_string()); // default use china mobile's cmnet
    run_fun!(
        mmcli -m $modem --simple-connect="apn=$apn"
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

pub fn disconnect(modem_id: Option<u32>) -> io::Result<()> {
    let modem = modem_id.unwrap_or(0);
    run_fun!(
        mmcli -m $modem --simple-disconnect
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
