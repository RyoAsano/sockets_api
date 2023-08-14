use std::{
    fmt::Display,
    mem::{self},
};

use libc::{c_void, INADDR_ANY};


use crate::api::fd::posix_fns::{htonl, inet_pton, htons};

use super::{safamily::SAFamily, SockAddrBase, sockaddr::SockAddrPtr};

pub fn new() -> SockAddrIn {
    SockAddrIn {
        sockaddr_in: unsafe { mem::zeroed::<libc::sockaddr_in>() },
        port: None,
        ip: None,
    }
}

pub struct SockAddrIn {
    sockaddr_in: libc::sockaddr_in,
    ip: Option<String>,
    port: Option<u16>,
}

impl Display for SockAddrIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", stringify!(self.ip), stringify!(self.port))
    }
}

impl SockAddrPtr for SockAddrIn {
    fn ptr_mut(&mut self) -> *mut libc::sockaddr_in {
        &mut self.sockaddr_in as *mut _
    }

    fn ptr(&self) -> *const libc::sockaddr_in {
        &self.sockaddr_in as *const _
    }

}

impl SockAddrBase for SockAddrIn {
    fn size(&self) -> u32 {
        mem::size_of_val(&self.sockaddr_in) as u32
    }

    fn ip(&self) -> Option<String> {
        self.ip.clone()
    }

    fn port(&self) -> Option<u16> {
        self.port
    }

    fn set(&mut self, af: SAFamily, ip: Option<&str>, port: u16) {
        self.sockaddr_in.sin_family = af as u8;

        // port
        self.port = Some(port);
        self.sockaddr_in.sin_port = unsafe { htons(port) };

        // IP address
        if let Some(ip) = ip {
            unsafe {
                if inet_pton(
                    af as i32,
                    ip.as_ptr(),
                    &mut self.sockaddr_in.sin_addr as *mut _ as *mut c_void,
                ) < 0
                {
                    panic!("Failed to convert the address: {}", ip);
                };
            }
            self.ip = Some(ip.to_string());
        } else {
            unsafe {
                self.sockaddr_in.sin_addr.s_addr = htonl(INADDR_ANY);
            }
            self.ip = Some("*".to_string());
        }
    }
}

