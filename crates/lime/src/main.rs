use clap::Parser;
use anyhow::Result;

mod args;
mod utils;

use crate::args::Arguments;
use crate::utils::Version;

fn main() -> Result<()> {
    let arg = Arguments::parse();
    
    let host = arg.host.or(Some("0.0.0.0".to_string())).unwrap();
    let port = arg.port.or(Some(9001)).unwrap();
    let debug = arg.debug.or(Some(true)).unwrap();
    let size = arg.size.or(Some(10)).unwrap();
    let ver = arg.ver.or(Some(env!("CARGO_PKG_VERSION").to_string())).unwrap();
    
    let version = Version::from(&ver)?;

    Ok(())
}
