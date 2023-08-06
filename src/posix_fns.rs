use libc::{c_void, size_t};

extern "C" {
    pub fn htons(x: u16) -> u16;
    pub fn htonl(x: u32) -> u32;
    pub fn inet_pton(af: i32, strptr: *const u8, addrptr: *mut c_void) -> i32;
    pub fn inet_ntop(af: i32, addrptr: *const c_void, strptr: *mut u8, len: size_t) -> *mut u8;

}