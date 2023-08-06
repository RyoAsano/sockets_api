use std::mem;

#[repr(i32)]
enum SAFamily {
    AfInet = libc::AF_INET,
    AfInet6 = libc::AF_INET6,
    AfLocal = libc::AF_LOCAL,
    AfLink = libc::AF_LINK,

}

pub struct SockAddrIn {
    sockaddr_in: libc::sockaddr_in,
}

impl SockAddrIn {
    fn new() -> Self {
        Self { 
            sockaddr_in: unsafe{ 
                mem::zeroed::<libc::sockaddr_in>() 
            } 
        }
    }

    fn set_af(&mut self, af: SAFamily) {
        self.sockaddr_in.sin_family = af as u8;
    }
}