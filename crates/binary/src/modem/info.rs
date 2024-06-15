use std::io;

use prettytable::row;
use prettytable::Table;
use struch::modemmanager::device::{ModemDevice, ModemDeviceList};

pub fn info<F>(f: F) -> io::Result<()>
where
    F: Fn(&ModemDevice) -> bool + 'static,
{
    let device_list = ModemDeviceList::new()?;
    let mut tab = Table::new();
    tab.add_row(row![
        Frb->"Device ID",
        Fgb->"Hardware",
        Fgb->"Name",
        Fyb->"NET IP",
        Fbb->"SIM IP",
        Fgb->"Connect",
    ]);
    for device in device_list.get_modem_device_list().iter() {
        if f(device) {
            if device.connected {
                tab.add_row(row![
                    Fr->device.device_id,
                    Fg->&device.hardware_name,
                    Fg->&device.device_name,
                    Fg->&device.net_ip,
                    Fb->&device.sim_ip,
                    Fg->&device.connected,
                ]);
                continue;
            }
            tab.add_row(row![
                Fr->device.device_id,
                Fg->&device.hardware_name,
                Fg->&device.device_name,
                Fg->&device.net_ip,
                Fb->&device.sim_ip,
                Fr->device.connected,
            ]);
        };
    }
    tab.printstd();
    Ok(())
}
