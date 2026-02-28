#[cfg(unix)]
pub fn stdout_is_tty() -> bool {
    use std::os::fd::AsRawFd;

    let fd = std::io::stdout().as_raw_fd();
    unsafe { libc::isatty(fd) == 1 }
}
