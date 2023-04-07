
use anyhow::{
    Result,
    Context
};

pub use crate::utils::version::Version;
pub mod version;


pub fn host_to_vec(host: &str) -> Result<Vec<u8>> {
    let mut h = Vec::new();
    let p = host.split(".")
        .collect::<Vec<&str>>();
    
    for x in p {
        h.push(
            x.parse::<u8>()
                .context("Failed to convert to u8. Perhaps your parameter is not formatted properly?")?
        );
    }

    Ok(h)
}
