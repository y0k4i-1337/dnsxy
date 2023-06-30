use std::net::SocketAddr;
use std::str::FromStr;

pub fn parse_proxy(proxy: &str) -> Result<SocketAddr, &'static str> {
    // Parse the proxy address and port
    match SocketAddr::from_str(proxy) {
        Ok(addr) => Ok(addr),
        Err(_) => Err("Invalid proxy address format"),
    }
}
