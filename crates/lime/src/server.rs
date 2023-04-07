use enet::*;
use std::{net::Ipv4Addr, time::Duration};
use anyhow::Context;
use colored::Colorize;
use log::{
    warn,
    info,
    debug,
    error
};


use crate::Version;

pub struct Server {
    host: Ipv4Addr,
    port: u16,
    peer_count: u64,
    enet: Enet,
    debug: bool,
    version: Version
}

#[allow(dead_code)]
enum Log {
    INFO,
    DEBUG,
    WARN,
    ERROR
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

    pub fn run_server(&self, duration: u64) {
        let mut host = self.enet.create_host::<()>(

            Some(&Address::new(self.host, self.port)),
            self.peer_count,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited

        ).context("Failed to create ENet host").unwrap();
        host.set_checksum_crc32(); 
        
        self.log(
            &format!("Server service {0}. Listening to {1}", 
                "started".green(), 
                "events".yellow()
            ), 
            Log::INFO
        );

        loop {
        
            if let Some(event) = host
                .service(Duration::from_secs(duration))
                .context("Service failure").unwrap()

            {
                self.log("Receive new event", Log::INFO);
                match event.r#type() {
                    
                    &EventType::Connect => self.handle_connection(event),
                    &EventType::Disconnect { .. } => println!("disconnect!"),
                    &EventType::Receive {
                        ref channel_id,
                        ref packet,
                    } => println!(
                        "got packet on channel {}, content: '{}'",
                        channel_id,
                        std::str::from_utf8(packet.data()).unwrap()
                    ),
                }
            }
        }
        
    }

    /// When the client first connect, send a handshake logic
    /// to determine if the connection is secure and legitimate.
    fn handle_connection(&self, event: Event<()>) {
        let peer = event.peer(); 
        self.log(
            &format!("New connection: {}:{}", 
                peer.address().ip(), 
                peer.address().port()
            ),
        Log::INFO);
        
    }

    

    fn log(&self, msg: &str, r#type: Log) {
        if self.debug {
            match r#type {
                Log::DEBUG => debug!("{}", msg),
                Log::WARN => warn!("{}", msg),
                Log::ERROR => error!("{}", msg),
                Log::INFO => info!("{}", msg),
            }
        }
    }
}
