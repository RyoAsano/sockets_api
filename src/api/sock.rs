use std::{fmt::Display, rc::Rc};

use libc::{sockaddr, SOCK_DGRAM, SOCK_RAW, SOCK_STREAM};

use crate::api::{
    addr::{safamily::SAFamily, sockaddr::SockAddrPtr, sockaddr_in::SockAddrIn},
    fd::Fd,
};

use super::{
    addr::SockAddrBase,
    fd::{socket_fd::SocketFd, FdWrapper},
};

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum SockType {
    SockStream = SOCK_STREAM,
    SockDgram = SOCK_DGRAM,
    SockRaw = SOCK_RAW,
}

#[derive(Debug)]
pub struct SockErr {
    msg: String,
}

impl Display for SockErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub enum State {
    New,
    Bound,
    Listen,
    Accept(Rc<SockAddrIn>),
}

pub struct Socket {
    fd: Option<Rc<Fd>>,
    state: State,
}

impl FdWrapper for Socket {
    fn fd(&self) -> Option<&Rc<Fd>> {
        if self.fd.is_none() {
            None
        } else {
            self.fd.as_ref()
        }
    }

    fn take_fd(&mut self) -> Option<Rc<Fd>> {
        self.fd.take()
    }

    fn set_fd(&mut self, fd: Rc<Fd>) {
        self.fd = Some(fd);
    }
}

impl Socket {
    pub fn new(af: SAFamily, ty: SockType) -> Box<Self> {
        let fd = Fd::new_socket(af, ty);
        if fd.is_ok() {
            return Box::new(Self {
                fd: Some(fd.ok().unwrap()),
                state: State::New,
            });
        } else {
            panic!("Error: {}", fd.err().unwrap());
        }
    }

    pub fn bind(self: Box<Self>, addr: &SockAddrIn) -> Box<Self> {
        match self.state {
            State::New => {
                let res = self.fd.as_ref().unwrap().bind(addr.ptr() as *const sockaddr, addr.size());
                if res.is_err() {
                    panic!("Error: {}", res.err().unwrap());
                }
                Box::new(Self {
                    fd: self.fd,
                    state: State::Bound,
                })
            }
            _ => {
                println!("The socket has already been bound.");
                self
            }
        }
    }

    pub fn listen(self: Box<Self>, backlog: i32) -> Result<Box<Self>, SockErr> {
        match self.state {
            State::Bound => {
                let res = self.fd.as_ref().unwrap().listen(backlog);
                if res.is_err() {
                    panic!("Error: {}", res.err().unwrap());
                }
                Ok(Box::new(Self {
                    fd: self.fd,
                    state: State::Listen,
                }))
            }
            State::New => Err(SockErr {
                msg: "Need to bind the socket before making it listening.".to_string(),
            }),
            _ => Ok(self),
        }
    }

    pub fn accept(self: &Box<Self>) -> Box<Self> {
        let res = self.fd.as_ref().unwrap().accept();

        if res.is_ok() {
            let (fd, cliaddr) = res.ok().unwrap();
            Box::new(Socket {
                fd: Some(fd),
                state: State::Accept(Rc::new(cliaddr)),
            })
        } else {
            panic!("Error: {}", res.err().unwrap());
        }
    }
}
