use std::{fmt::Display, rc::Rc};

use libc::{sockaddr, socket};

use crate::api::{
    addr::{
        safamily::SAFamily,
        sockaddr::SockAddrPtr,
        sockaddr_in::{self, SockAddrIn},
        SockAddrBase,
    },
    sock::SockType,
};

use super::{posix_fns::inet_ntop, Fd};

pub struct SockErr {
    msg: String,
}

impl Display for SockErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub trait SocketFd {
    fn new_socket(af: SAFamily, ty: SockType) -> Result<Rc<Self>, SockErr>;

    fn bind(&self, addr_ptr: *const sockaddr, len: u32) -> Result<(), SockErr>;

    fn listen(&self, backlog: i32) -> Result<(), SockErr>;

    fn accept(&self) -> Result<(Rc<Self>, SockAddrIn), SockErr>;
}

impl SocketFd for Fd {
    fn new_socket(af: SAFamily, ty: SockType) -> Result<Rc<Self>, SockErr> {
        let raw_fd;
        unsafe {
            raw_fd = socket(af as i32, ty as i32, 0);
        }
        if raw_fd < 0 {
            Err(SockErr {
                msg: String::from("Failed to create a new socket."),
            })
        } else {
            Ok(Rc::new(Self { raw_fd }))
        }
    }

    fn bind(&self, addr_ptr: *const sockaddr, len: u32) -> Result<(), SockErr> {
        if unsafe { libc::bind(self.raw_fd, addr_ptr, len) } < 0 {
            Err(SockErr {
                msg: String::from("Failed to bind the socket."),
            })
        } else {
            Ok(())
        }
    }

    fn listen(&self, backlog: i32) -> Result<(), SockErr> {
        if unsafe { libc::listen(self.raw_fd, backlog) } < 0 {
            Err(SockErr {
                msg: String::from("Failed to make the socket listen."),
            })
        } else {
            Ok(())
        }
    }

    fn accept(&self) -> Result<(Rc<Self>, SockAddrIn), SockErr> {
        let mut cliaddr = sockaddr_in::new();
        let len = &mut 0u32 as *mut _;

        let raw_fd = unsafe { libc::accept(self.raw_fd, cliaddr.ptr_mut() as *mut sockaddr, len) };

        if raw_fd < 0 {
            Err(SockErr {
                msg: String::from("Failed to accept."),
            })
        } else {
            let mut addrstr: [u8; 15] = [0u8; 15];
            if unsafe {
                inet_ntop(
                    SAFamily::AfInet as i32,
                    cliaddr.ptr() as *const _,
                    addrstr.as_mut_ptr(),
                    cliaddr.size() as usize,
                )
            }
            .is_null()
            {
                panic!("Failed to convert the numeric into string.");
            }
            let addrstr = addrstr
                .iter()
                .filter(|&&c| c != 0u8)
                .map(|&x| x as char)
                .collect::<String>();
            cliaddr.set(SAFamily::AfInet, Some(addrstr.as_str()), 0);

            Ok((Rc::new(Self { raw_fd }), cliaddr))
        }
    }
}
