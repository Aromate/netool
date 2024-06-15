pub fn geteuid() -> u32 {
    unsafe { libc::geteuid() }
}

pub fn getuid() -> u32 {
    unsafe { libc::getuid() }
}
