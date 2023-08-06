use libc::{AF_INET, c_void};
use sockets_api::{posix_fns::{htons, inet_pton}, addr::{self, IpV4AddrSvc, IpV4Addr}};


fn main() {
    unsafe {
        let addrnum = IpV4Addr::inet_pton("127.0.0.1");
        println!("addrnum={}", addrnum);

        // let addrp = IpV4Addr::inet_ntop(addrnum);
        // println!("addrp={}", addrp);

        let x = htons(9877);
        println!("x={}", x);
    }
}