use enet::*;
use std::{net::Ipv4Addr, time::Duration};
use anyhow::Context;

use crate::utils::Version;

pub struct Server {
    host: Ipv4Addr,
    port: u16,
    peer_count: u64,
    enet: Enet,
    debug: bool,
    version: Version
}

impl Server {
    pub fn new(
        host: Ipv4Addr,
        port: u16,
        peer_count: u64,
        debug: bool,
        v: Version
    ) -> Self {

        let enet_obj = Enet::new().context("Failed to initialize ENet object").unwrap();
        Server {
            host: host,
            port: port,
            peer_count: peer_count,
            debug: debug,
            version: v,
            enet: enet_obj
        }
    }
}
