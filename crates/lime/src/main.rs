use std::net::Ipv4Addr;
use clap::Parser;
use anyhow::{
    Result,
    Context
};

mod args;
mod utils;
mod server;
mod address;

use crate::address::Address;
use crate::args::Arguments;
use crate::utils::Version;
use crate::server::Server;

fn main() -> Result<()> {
    let arg = Arguments::parse();
    
    let host = arg.host.or(Some("0.0.0.0".to_string())).unwrap();    
    let port = arg.port.or(Some(9001)).unwrap();

    let debug = arg.debug.or(Some(true)).unwrap();
    let size = arg.size.or(Some(10)).unwrap();
    let ver = arg.ver.or(Some(env!("CARGO_PKG_VERSION").to_string())).unwrap();
    
    let addr = Address::new(&host, port);

    let version = Version::from(&ver)?;
    let server = Server::new(
        addr.host,
        addr.port,
        size,
        debug,
        version
    );

    Ok(())
}
