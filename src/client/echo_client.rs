use std::io;

use libc::{socket, AF_INET, SOCK_STREAM, connect, write, c_void, close, read};

use crate::addr::{IpV4Addr, IpV4AddrSvc};



pub fn mux_connect() {
    unsafe {
        let mut server_addr = IpV4Addr::new();
        let paddr = "127.0.0.1".to_string();
        server_addr.set(IpV4Addr::inet_pton(&paddr), 9877);

        let sock = socket(AF_INET, SOCK_STREAM, 0);

        let a = server_addr.ptr_as_sockaddr();
        println!("{:?}", a);
        let a = IpV4Addr::inet_pton(&paddr);
        println!("{:#034b}", a);
        let connfd = connect(sock, server_addr.ptr_as_sockaddr(), IpV4Addr::size());

        loop {
            let mut buf = String::new();
            if let Ok(n) = io::stdin().read_line(&mut buf) {
                write(connfd, buf.as_ptr() as *const c_void, n);

                let read_buf = String::new();
                let _ = read(connfd, read_buf.as_ptr() as *mut c_void, 1024);

                println!("{}", read_buf);
            } else {
                close(connfd);
            };
        };
    }
}