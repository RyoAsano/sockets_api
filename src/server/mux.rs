use std::{io::Error, collections::VecDeque};

use crate::api::{addr::{sockaddr_in::{SockAddrIn, self}, safamily::SAFamily, SockAddrBase}, sock::{SockType, Socket}, fd::{FdSet, FdWrapper, io_fd::IoFd}};


pub struct IoMuxServer {
    sock: Option<Box<Socket>>,
    addr: SockAddrIn,
    clis: VecDeque<Box<Socket>>,
}

impl IoMuxServer {
    pub fn new() -> Self {
        let mut addr = sockaddr_in::new();
        addr.set(SAFamily::AfInet, None, 9877);

        Self {
            sock: Some(Socket::new(SAFamily::AfInet, SockType::SockStream)),
            addr: addr,
            clis: VecDeque::new(),
        }
    }

    pub fn launch(&mut self) -> Result<(), Error> {
        self.sock = Some(self.sock.take().unwrap().bind(&self.addr));
        self.sock = Some(self.sock.take().unwrap().listen(1024).expect("Something wrong from Bound to Listen."));

        let mut fdsets = FdSet::new();
        fdsets.set(self.sock.as_ref().unwrap().as_ref());

        loop {
            let mut rfds = fdsets.clone();
            FdSet::select(Some(&mut rfds), None, None);

            if rfds.is_set(self.sock.as_ref().unwrap().as_ref()) {
                let conn_sock = self.sock.as_ref().unwrap().accept();
                fdsets.set(conn_sock.as_ref());
                self.clis.push_back(conn_sock);
            }

            for _ in 0..self.clis.len() {
                let mut conn_sock = self.clis.pop_front().unwrap();

                if rfds.is_set(conn_sock.as_ref()) {
                    if let Some((fd, msg)) = conn_sock.take_fd().unwrap().read() {
                        fd.write(msg.as_str());
                        conn_sock.set_fd(fd);
                        self.clis.push_back(conn_sock);
                    }
                } else {
                    self.clis.push_back(conn_sock);
                }

            } 
        }
   }
}
