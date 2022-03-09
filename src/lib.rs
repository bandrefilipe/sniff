use clap::Parser;
use std::error::Error;
use std::net::IpAddr;

pub fn run_application() -> Result<(), Box<dyn Error>> {
    let _ = Args::parse();
    todo!("implement application logic")
}

/// A simple IP Sniffer tool written in Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// An IPv4 or IPv6 valid IP address
    ip_address: IpAddr,

    /// The number of threads to use
    #[clap(short, long, default_value_t = 4)]
    threads: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "implement application logic")]
    fn application_panics() {
        run_application().unwrap();
    }
}
