mod notification;

use clap::Parser;
use std::error::Error;
use std::net::{IpAddr, TcpStream};
use std::thread;
use crate::notification::{NotificationChannels, NotificationProducer};

pub fn run_application() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let channels = NotificationChannels::new();

    for i in 1..=args.threads {
        let notifier = channels.new_notification_producer();
        thread::spawn(move || {
            let address = args.ip_address;
            let start_port = i;
            let increment = args.threads;
            ScannerService::from(notifier, address, start_port, increment).scan();
        });
    }

    let consumer = channels.new_notification_consumer();
    consumer.print_result();

    Ok(())
}

struct ScannerService {
    notifier: NotificationProducer,
    address: IpAddr,
    start_port: u16,
    increment: u16,
}

impl ScannerService {
    fn from(
        notifier: NotificationProducer,
        address: IpAddr,
        start_port: u16,
        increment: u16,
    ) -> ScannerService {
        ScannerService {
            notifier,
            address,
            start_port,
            increment,
        }
    }

    /// Loops through each port, from `start_port` up to [u16::MAX], incrementing the port number by
    /// the `increment` value in each iteration, and sends each connected port number through the
    /// channel.
    fn scan(&self) {
        let mut port = self.start_port;
        loop {
            match TcpStream::connect((self.address, port)) {
                Ok(_) => self.notifier.connection_succeeded(port),
                Err(_) => self.notifier.connection_failed(port),
            }

            if (u16::MAX - port) <= self.increment {
                // breaks from the loop if incrementing the port would exceed the max value for u16
                break;
            }

            port += self.increment;
        }
    }
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
