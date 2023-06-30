mod proxy;

use std::error::Error;
use clap::{Parser};

#[derive(Debug, Parser)]
#[command(author, about, version, long_about = None)]
struct Cli {
    /// Nameservers to use (comma-separated list or file)
    #[arg(short, long)]
    nameservers: String,

    /// SOCKS proxy to use (format: "address:port")
    #[arg(short = 'x', long)]
    proxy: String,

    /// Server address to listen on
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    /// Server port to listen on
    #[arg(short, long, default_value_t = 53, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    // println!("{:?}", cli);

    // Parse the SOCKS proxy address and port
    let proxy_addr = proxy::parse_proxy(&cli.proxy)?;

    println!("Listening on {}:{}", cli.address, cli.port);
    println!("Using proxy {}", proxy_addr);

    Ok(())
}
