use std::net::Ipv4Addr;

use crate::utils::host_to_vec;

pub struct Address {
    pub host: Ipv4Addr,
    pub port: u16
}

impl Address {
    pub fn new(h: &str, p: u16) -> Self {
        
        let hv = host_to_vec(h).unwrap();

        Address {
            host: Ipv4Addr::new(hv[0], hv[1], hv[2], hv[3]),
            port: p
        }
    }
   
}

