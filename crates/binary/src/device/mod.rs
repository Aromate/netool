use std::io;

use prettytable::{row, Table};
use struch::iproute2::{Device, DeviceList};

pub fn info<F>(f: F) -> io::Result<()>
where
    F: Fn(&Device) -> bool + 'static,
{
    let device_list = DeviceList::new()?;
    let mut tab = Table::new();
    tab.add_row(row![
        Frb->"Device ID",
        Fgb->"Name",
        Fgb->"Type",
        Fgb->"MAC",
        Fgb->"State",
        Frb->"IP"
    ]);
    macro_rules! color {
        ($style:ident,$device:expr) => {
            tab.add_row(row![
                    Fr->$device.ifindex,
                    Fb->$device.ifname,
                    Fg->$device.link_type,
                    Fg->$device.address.as_ref().unwrap_or(&String::new()),
                    $style->$device.operstate,
                    Fg->&$device.get_ip_addr()
                ]);
        };
    }
    for device in device_list.get_address_list().iter() {
        if f(device) {
            if device.operstate == "UP" {
                color!(Fg, device);
                continue;
            }
            if device.operstate == "DOWN" {
                color!(Fr, device);
                continue;
            }
            color!(Fy, device);
        };
    }
    tab.printstd();
    Ok(())
}

#[macro_export]
macro_rules! filter {
    ($key:ident,$value:expr) => {
        |device: &Device| device.$key == $value
    };
}

pub fn default_filter(_device: &Device) -> bool {
    true
}
