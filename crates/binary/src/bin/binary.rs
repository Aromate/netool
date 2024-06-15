use std::io;

use binary::{device, modem};
use struch::modemmanager::bearer::BearerInfo;
use struch::modemmanager::device::ModemDevice;
use struch::{iproute2::Device, modemmanager::device::ModemDeviceList};

use clap::Parser;
use struch::modemmanager::modem::{ModemInfo, PortList};

#[derive(Parser)]
#[clap(about = "A simple tool to manage net device")]
pub enum Command {
    #[clap(about = "List all devices")]
    ListDevice(DeviceFilter),
    #[clap(about = "List all modems")]
    ListModem(ModemFilter),
    #[clap(about = "Auto connect lte")]
    Lte(Lte),
}

#[derive(Parser)]
pub struct Lte {
    #[clap(short, long, default_value = "false")]
    disconnect: bool,
    #[clap(short, long, default_value = "false")]
    connect: bool,
    #[clap(short, long, default_value = "0")]
    modem_id: u32,
    #[clap(short, long, default_value = "cmnet")]
    apn: String,
}

#[derive(Parser)]
pub struct DeviceFilter {
    #[clap(short, long)]
    id: Option<u32>,
    #[clap(short, long)]
    name: Option<String>,
    #[clap(short, long)]
    link_type: Option<String>,
}

#[derive(Parser)]
pub struct ModemFilter {
    /// Modem device id
    #[clap(short, long)]
    id: Option<u32>,

    /// Net device name
    #[clap(short, long)]
    name: Option<String>,

    /// Modem connect status:
    /// connected or disconnected
    #[clap(short, long)]
    state: Option<String>,
}

fn main() -> io::Result<()> {
    let cmd = Command::parse();
    match cmd {
        Command::ListModem(filter) => {
            let filter = move |device: &ModemDevice| {
                if let Some(id) = filter.id {
                    if device.device_id != id {
                        return false;
                    }
                }
                if let Some(name) = &filter.name {
                    if device.device_name != *name {
                        return false;
                    }
                }
                if let Some(state) = &filter.state {
                    if device.connected && state == "connected" {
                        return true;
                    }
                    if !device.connected && state == "disconnected" {
                        return true;
                    }
                    return false;
                }
                true
            };
            modem::info(filter)?
        }
        Command::ListDevice(filter) => {
            let filter = move |device: &Device| {
                if let Some(ifindex) = filter.id {
                    if device.ifindex != ifindex {
                        return false;
                    }
                }
                if let Some(ifname) = &filter.name {
                    if device.ifname != *ifname {
                        return false;
                    }
                }
                if let Some(link_type) = &filter.link_type {
                    if device.link_type != *link_type {
                        return false;
                    }
                }
                true
            };
            device::info(filter)?
        }
        Command::Lte(lte) => {
            if lte.connect && lte.disconnect {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Can't connect and disconnect at the same time",
                ));
            }
            if utils::system::getuid() != 0 || utils::system::geteuid() != 0 {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "Permission denied",
                ));
            }
            let modem_id = if lte.modem_id == 0 {
                lte.modem_id.to_string()
            } else {
                let modem_device_list = ModemDeviceList::new()?;
                modem_device_list
                    .find_by_id(lte.modem_id)
                    .unwrap()
                    .modem
                    .modem
                    .dbus_path
                    .to_string()
            };
            if lte.connect {
                let mut modem_info = ModemInfo::new(&modem_id)?;
                if modem_info.modem.generic.state != "connected" {
                    modem_info.connect(&lte.apn)?;
                }
                modem_info.refresh()?;
                let bearer_id = modem_info.modem.generic.bearers.first().unwrap();
                let bearer_info = BearerInfo::new(bearer_id)?;
                let device_name = bearer_info.bearer.status.interface;
                let mut device = Device::new(&device_name)?;
                device.flush()?;
                device.refresh()?;
                let ip_v4 = format!("{}/32", bearer_info.bearer.ipv4_config.address);
                device.add_ip(&ip_v4)?;
                device.set_up()?;
                device.set_ip_route()?;
                println!("sucess");
            }
            if lte.disconnect {
                let modem_info = ModemInfo::new(modem_id)?;
                modem_info.disconnect()?;
                let ports = PortList::from_vec_string(&modem_info.modem.generic.ports);
                let device_name = ports.get_net_name();
                let device = Device::new(&device_name)?;
                device.del_route()?;
                device.flush()?;
                device.set_down()?;
                println!("sucess");
            }
        }
    }
    Ok(())
}
