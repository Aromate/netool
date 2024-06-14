use std::io;

use crate::ip;

use prettytable::{color, row, Attr, Cell, Table};

pub fn print_devices_addr() -> io::Result<()> {
    let devices_addr = ip::get_all_address()?;
    let mut tab = Table::new();
    tab.add_row(row![
        Cell::new("Device")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new("Link Type")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("MTU")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BLUE)),
        Cell::new("Addresses")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::YELLOW)),
    ]);
    for address in devices_addr.get_address_list() {
        let addrs = address
            .addr_info
            .iter()
            .filter(|addr| addr.family == "inet")
            .map(|addr| addr.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        tab.add_row(row![
            address.ifname.clone(),
            address.link_type.clone(),
            address.mtu,
            addrs,
        ]);
    }
    tab.printstd();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_addr() {
        print_devices_addr().unwrap();
    }
}
