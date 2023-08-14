use std::{mem::size_of_val, rc::Rc};

use libc::{c_void, read, write};
use log::info;

use super::Fd;

const MAXLINE: usize = 4096;

pub trait IoFd {
    fn read(self: Rc<Self>) -> Option<(Rc<Self>, String)>;
    fn write(&self, msg: &str);
}

impl IoFd for Fd {
    fn read(self: Rc<Self>) -> Option<(Rc<Self>, String)> {
        let mut buf: [u8; MAXLINE] = [0; MAXLINE];
        let n = unsafe {
            read(
                self.raw_fd,
                buf.as_mut_ptr() as *mut c_void,
                size_of_val(&buf),
            )
        };
        if n == 0 {
            info!("Connection closed.");
            None
        } else {
            let msg = buf
                .iter()
                .filter(|&&c| c != 0u8)
                .map(|&c| c as char)
                .collect::<String>();
            info!("Received: {}", msg);
            Some((self, msg))
        }
    }

    fn write(&self, msg: &str) {
        unsafe {
            write(
                self.raw_fd,
                msg.as_bytes().as_ptr() as *const c_void,
                size_of_val(msg),
            );
        }
    }
}
