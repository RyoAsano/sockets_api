use std::{mem, ptr::null_mut, rc::Rc};

use libc::{close, fd_set, select, FD_ISSET, FD_SET, FD_ZERO};
use log::info;

pub(super) mod posix_fns;
pub mod io_fd;
pub mod socket_fd;

pub struct Fd {
    raw_fd: i32,
}

impl Fd {
    pub fn close(&self) {
        info!("Closing the fd: {}", self.raw_fd);
        unsafe {
            close(self.raw_fd);
        }
    }
}

impl Drop for Fd {
    fn drop(&mut self) {
        self.close();
    }
}

pub trait FdWrapper {
    fn fd(&self) -> Option<&Rc<Fd>>;
    fn take_fd(&mut self) -> Option<Rc<Fd>>;
    fn set_fd(&mut self, fd: Rc<Fd>);
}

#[derive(Clone)]
pub struct FdSet {
    raw_fdset: fd_set,
    max: i32,
}

impl FdSet {
    pub fn new() -> Self {
        let mut fd_set = unsafe { mem::zeroed::<fd_set>() };
        unsafe {
            FD_ZERO(&mut fd_set);
        }
        Self {
            raw_fdset: fd_set,
            max: 0,
        }
    }

    pub fn set(&mut self, fdwrapper: &impl FdWrapper) {
        let fd = fdwrapper.fd().unwrap().raw_fd;
        unsafe {
            FD_SET(fd, &mut self.raw_fdset);
        }
        if self.max < fd + 1 {
            self.max = fd + 1;
        }
    }

    pub fn is_set(&self, fdwrapper: &impl FdWrapper) -> bool {
        unsafe { FD_ISSET(fdwrapper.fd().unwrap().raw_fd, &self.raw_fdset) }
    }

    pub fn select(r_fds: Option<&mut Self>, w_fds: Option<&mut Self>, e_fds: Option<&mut Self>) {
        let mut nfds = 0;

        let r_ptr;
        if let Some(r_fds) = r_fds {
            r_ptr = &mut r_fds.raw_fdset as *mut _;
            nfds = r_fds.max;
        } else {
            r_ptr = null_mut() as *mut fd_set;
        }

        let w_ptr;
        if let Some(w_fds) = w_fds {
            w_ptr = &mut w_fds.raw_fdset as *mut _;
            if w_fds.max > nfds {
                nfds = w_fds.max;
            }
        } else {
            w_ptr = null_mut() as *mut fd_set;
        }

        let e_ptr;
        if let Some(e_fds) = e_fds {
            e_ptr = &mut e_fds.raw_fdset as *mut _;
            if e_fds.max > nfds {
                nfds = e_fds.max;
            }
        } else {
            e_ptr = null_mut() as *mut fd_set;
        }

        unsafe {
            if select(nfds, r_ptr, w_ptr, e_ptr, null_mut()) < 0 {
                panic!("Something wrong happened.");
            };
        }
    }
}
