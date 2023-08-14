pub trait SockAddrPtr {
    fn ptr_mut(&mut self) -> *mut libc::sockaddr_in;
    fn ptr(&self) -> *const libc::sockaddr_in;
}

