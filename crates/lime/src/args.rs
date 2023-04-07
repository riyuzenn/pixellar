
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    
    /// The host of the server to be binded to.
    /// Default: 0.0.0.0
    #[arg(short, long)]
    pub host: Option<String>,

    /// The port of the server to be binded to.
    /// Default: 9001
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Number of client the server will accept
    /// Default Value: 10 connections
    #[arg(short, long)]
    pub size: Option<u64>,

    /// Set the debug to true or false
    /// Default: true
    #[arg(short, long)]
    pub debug: Option<bool>,
    
}
