use std::fmt::Display;

use self::{sockaddr::SockAddrPtr, safamily::SAFamily};

pub mod sockaddr_in;
pub mod sockaddr;
pub mod safamily;

pub trait SockAddrBase: SockAddrPtr {
    fn set(&mut self, af: SAFamily, ip: Option<&str>, port: u16);
    fn port(&self) -> Option<u16>;
    fn ip(&self) -> Option<String>;
    fn size(&self) -> u32;
}

pub trait SockAddr: SockAddrBase + Display {}

