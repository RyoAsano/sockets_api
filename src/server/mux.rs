use std::{collections::HashMap, io::Error, mem, ptr::null_mut};

use libc::{
    accept, bind, c_void, close, fd_set, listen, read, select, socket, write, AF_INET, FD_ISSET,
    FD_SET, FD_ZERO, INADDR_ANY, SOCK_STREAM,
};
use log::info;

use crate::{addr::{IpV4Addr, IpV4AddrSvc}, posix_fns::{htons, htonl}};

type Fd = i32;

pub struct IoMuxServer {
    addr: IpV4Addr,
    listenfd: Fd,
    clis: HashMap<Fd, IpV4Addr>,
}

pub trait IoMuxServerSvc {
    fn new() -> Self;
    fn listen(&mut self);
    fn launch(&mut self) -> Result<(), Error>;
}

impl IoMuxServerSvc for IoMuxServer {
    fn new() -> Self {
        Self {
            addr: IpV4Addr::new(),
            listenfd: -1,
            clis: HashMap::new(),
        }
    }

    fn listen(&mut self) {
        unsafe {
            self.listenfd = socket(AF_INET, SOCK_STREAM, 0);
            self.addr.set(htonl(INADDR_ANY), htons(9877));

            if bind(self.listenfd, self.addr.ptr_as_sockaddr(), IpV4Addr::size()) < 0 {
                panic!("Failed to bind the socket");
            };

            if listen(self.listenfd, 1024) < 0 {
                panic!("Failed to let the socket listen.");
            };
        }
    }

    fn launch(&mut self) -> Result<(), Error> {
        unsafe {
            let mut allset = mem::zeroed::<fd_set>();
            FD_ZERO(&mut allset);
            FD_SET(self.listenfd, &mut allset);

            let mut maxfd = self.listenfd + 1;
            
            loop {
                let mut rset = allset.clone();

                println!("maxfd={}", maxfd);
                let _ = select(maxfd, &mut rset, null_mut(), null_mut(), null_mut());

                if FD_ISSET(self.listenfd, &rset) {
                    println!("Here it comes.");
                    let mut cli_addr = IpV4Addr::new();
                    let mut size = 0;
                    let connfd = accept(self.listenfd, cli_addr.ptr_as_sockaddr_mut(), &mut size);
                    
                    if connfd < 0 {
                        println!("Failed to accept.");
                    }
                    println!("Added a client of clifd={}, addr={}", connfd, cli_addr.ip());
                    self.clis.insert(connfd, cli_addr);
                    FD_SET(connfd, &mut allset);

                    if connfd + 1 > maxfd {
                        maxfd = connfd + 1
                    };
                }

                for (&clifd, &ref cliaddr) in &self.clis {
                    println!("clifd={}, cliaddr={}", clifd, cliaddr.ip());

                    if FD_ISSET(clifd, &rset) {
                        println!("Serving the client of ip={}", cliaddr.ip());

                        let mut buff: [u8; 10000] = [0; 10000];
                        let n = read(clifd, &mut buff as *mut u8 as *mut c_void, 10000);
                        if n == 0 {
                            info!("Connection closed.");
                            close(clifd);
                        } else if n > 0 {
                            println!("n={}", n);
                            let msg = buff.iter().filter(|&&x| x != 0u8).map(|&x| x as char).collect::<String>();

                            println!("Received {}", msg);
                            write(clifd, msg.as_ptr() as *const c_void, msg.len());
                        }
                    }
                }
            }
        };
    }
}
