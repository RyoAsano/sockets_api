#[derive(Copy, Clone)]
#[repr(i32)]
pub enum SAFamily {
    AfInet = libc::AF_INET,
    AfInet6 = libc::AF_INET6,
    AfLocal = libc::AF_LOCAL,
    AfLink = libc::AF_LINK,
}