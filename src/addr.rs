pub mod sockaddr_in;


use std::mem::{self, size_of};

use libc::{in_addr_t, AF_INET, c_void};

use crate::posix_fns;

pub struct IpV4Addr {
    addr_in: libc::sockaddr_in,
}

pub trait IpV4AddrSvc {
    fn new() -> Self;
    fn set(&mut self, addr: in_addr_t, port: u16);
    fn ptr_as_sockaddr(&self) -> *const libc::sockaddr;
    fn ptr_as_sockaddr_mut(&mut self) -> *mut libc::sockaddr;
    fn ip(&self) -> String;

    fn size() -> u32 {
        size_of::<libc::sockaddr_in>() as u32
    }

    fn inet_ntop(addr: & libc::sockaddr_in) -> String {
        let addr = addr as *const _;
        unsafe {
            // 15 is the maximum length of ipv4 presentation
            // e.g. "255.255.255.255" is of length 15.
            let mut hoge: [u8; 15] = [0u8; 15]; 

            if posix_fns::inet_ntop(AF_INET, addr as *const c_void, hoge.as_mut_ptr(), hoge.len()).is_null() {
                panic!("Falied to convert the numeric into presentation");
            };
            hoge.iter().map(|&x| x as char).collect::<String>()
        }
    }

    fn inet_pton(addr: &str) -> u32 {
        let strptr = addr.to_string().as_ptr();
        let addrptr = &mut 0u32 as *mut u32;
        unsafe {
            if posix_fns::inet_pton(AF_INET, strptr, addrptr as *mut c_void) < 0 {
                panic!("Falied to convert the IP address: {}", addr);
            };
            *addrptr
        }
    }

}

impl IpV4AddrSvc for IpV4Addr {
    fn new() -> Self {
        let mut addr_in = unsafe { mem::zeroed::<libc::sockaddr_in>() };
        addr_in.sin_family = AF_INET as u8;
        IpV4Addr { addr_in }
    }

    fn set(&mut self, addr: in_addr_t, port: u16) {
        self.addr_in.sin_addr.s_addr = addr;
        self.addr_in.sin_port = port;
    }

    fn ptr_as_sockaddr(&self) -> *const libc::sockaddr {
        &self.addr_in as *const libc::sockaddr_in as *const libc::sockaddr
    }

    fn ptr_as_sockaddr_mut(&mut self) -> *mut libc::sockaddr {
        &mut self.addr_in as *mut libc::sockaddr_in as *mut libc::sockaddr
    }

    fn ip(&self) -> String {
        IpV4Addr::inet_ntop(&self.addr_in)
    }
}
