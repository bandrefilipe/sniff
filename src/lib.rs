use clap::Parser;
use std::error::Error;
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub fn run_application() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let (sender, receiver) = channel();

    for i in 1..=args.threads {
        // each thread will receive a clone of the sender channel
        let sender = sender.clone();
        thread::spawn(move || {
            let start_port = i;
            let increment = args.threads;
            ScannerService::from(sender, args.ip_address, start_port, increment).scan();
        });
    }
    drop(sender); // drops the original sender channel because it's now useless

    print_result(receiver);

    Ok(())
}

/// Process the given `receiver` channel and prints the result.
fn print_result(receiver: Receiver<u16>) {
    let mut ports = Vec::new();
    for port in receiver {
        ports.push(port);
    }

    let n = ports.len();
    match n {
        0 => println!("{n} open ports found."),
        1 => println!("{n} open port found:"),
        _ => println!("{n} open ports found:"),
    };

    ports.sort_unstable();
    for port in ports {
        println!("\t{port}");
    }
}

/// Responsible for scanning open ports from an IP address.
///
/// New instances can be created with the factory function [ScannerService::from].
struct ScannerService {
    sender: Sender<u16>,
    address: IpAddr,
    start_port: u16,
    increment: u16,
}

impl ScannerService {
    /// Creates a new `ScannerService` from a sender channel, ip address, start port and increment.
    ///
    /// * `sender`: the channel to which successfully scanned ports are sent.
    /// * `address`: the IP address from which the ports will be scanned.
    /// * `start_port`: describes from which port the scan will begin.
    /// * `increment`: describes how much to increment the port number in each scan.
    fn from(
        sender: Sender<u16>,
        address: IpAddr,
        start_port: u16,
        increment: u16,
    ) -> ScannerService {
        ScannerService {
            sender,
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
            if TcpStream::connect((self.address, port)).is_ok() {
                self.sender.send(port).unwrap();
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
