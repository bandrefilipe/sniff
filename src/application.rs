use crate::notification::NotificationFactory;
use crate::scan::ScannerService;
use clap::Parser;
use std::error::Error;
use std::net::IpAddr;
use std::thread;

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

/// Runs the application logic.
pub fn run_application() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let factory = NotificationFactory::new();

    // assign threads to scan the IP address
    for i in 1..=args.threads {
        let notifier = factory.new_notification_producer();
        thread::spawn(move || {
            let address = args.ip_address;
            let start_port = i;
            let increment = args.threads;
            ScannerService::from(notifier, address, start_port, increment).scan();
        });
    }

    let consumer = factory.new_notification_consumer();
    consumer.print_result();

    Ok(())
}
