use crate::notification::NotificationProducer;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

/// Responsible for scanning the open ports of an IP address.
pub struct ScannerService {
    notifier: NotificationProducer,
    address: IpAddr,
    start_port: u16,
    increment: u16,
    timeout: Duration,
}

impl ScannerService {
    /// Creates a new `ScannerService` from the given inputs.
    pub fn from(
        notifier: NotificationProducer,
        address: IpAddr,
        start_port: u16,
        increment: u16,
        timeout: u64,
    ) -> ScannerService {
        ScannerService {
            notifier,
            address,
            start_port,
            increment,
            timeout: Duration::from_millis(std::cmp::max(1, timeout)),
        }
    }

    /// Creates a new [SocketAddr] from the current IP address and the given `port`.
    fn socket_addr(&self, port: u16) -> SocketAddr {
        SocketAddr::new(self.address, port)
    }

    /// Loops through each port, from `start_port` up to [u16::MAX], incrementing the port number by
    /// the `increment` value in each iteration, and notifies the progression through the `notifier`.
    pub fn scan(&self) {
        let mut port = self.start_port;
        loop {
            match TcpStream::connect_timeout(&self.socket_addr(port), self.timeout) {
                Ok(_) => self.notifier.connection_succeeded(port),
                Err(_) => self.notifier.connection_failed(port),
            }

            if (u16::MAX - port) < self.increment {
                // breaks from the loop if incrementing the port would exceed the max value for u16
                break;
            }

            port += self.increment;
        }
    }
}
