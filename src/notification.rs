use progress_bar::color::{Color, Style};
use progress_bar::progress_bar::ProgressBar;
use std::sync::mpsc::{channel, Receiver, Sender};

/// Responsible for creating the [NotificationProducer] and [NotificationConsumer].
pub struct NotificationFactory {
    success_channels: (Sender<u16>, Receiver<u16>),
    failure_channels: (Sender<u16>, Receiver<u16>),
}

impl NotificationFactory {
    /// Creates a new `NotificationFactory`.
    pub fn new() -> Self {
        Self {
            success_channels: channel(),
            failure_channels: channel(),
        }
    }

    /// Intermediate operation.
    ///
    /// Creates a new [NotificationProducer].
    pub fn new_notification_producer(&self) -> NotificationProducer {
        // clone the sender halves
        let success_channel = self.success_channels.0.clone();
        let failure_channel = self.failure_channels.0.clone();

        NotificationProducer {
            success_channel,
            failure_channel,
        }
    }

    /// Terminal operation.
    ///
    /// Creates a new [NotificationConsumer] and destroys this `NotificationFactory` instance.
    pub fn new_notification_consumer(self) -> NotificationConsumer {
        let success_channel = self.success_channels.1;
        let failure_channel = self.failure_channels.1;

        NotificationConsumer::from(success_channel, failure_channel)
    }
}

/// Notifies the progress of the application through the appropriate channels.
pub struct NotificationProducer {
    success_channel: Sender<u16>,
    failure_channel: Sender<u16>,
}

impl NotificationProducer {
    /// Notifies that a connection to the given `port` has been successfully established.
    pub fn connection_succeeded(&self, port: u16) {
        self.success_channel.send(port).unwrap();
    }

    /// Notifies that a connection to the given `port` has failed.
    pub fn connection_failed(&self, port: u16) {
        self.failure_channel.send(port).unwrap();
    }
}

/// Consumes application' progress notifications through the appropriate channels.
pub struct NotificationConsumer {
    success_channel: Receiver<u16>,
    failure_channel: Receiver<u16>,
}

impl NotificationConsumer {
    /// Creates a new `NotificationConsumer` from the given `channels`.
    fn from(success_channel: Receiver<u16>, failure_channel: Receiver<u16>) -> Self {
        Self {
            success_channel,
            failure_channel,
        }
    }

    /// Consumes the notifications and prints the application result.
    pub fn print_result(self) {
        // conveniently put the channels into this scope
        let failure_channel = self.failure_channel;
        let success_channel = self.success_channel;

        // set the progress bar
        let mut progress_bar = ProgressBar::new(100);
        progress_bar.set_action("Sniffing", Color::Cyan, Style::Bold);

        // to be incremented by 1 for each notification consumed (success of failure)
        let mut progress = 0;

        // process failures
        for _ in failure_channel {
            progress += 1;
            if progress % (u16::MAX / 100) == 0 {
                progress_bar.inc();
            }
        }

        // process successes
        let mut ports = Vec::new();
        for port in success_channel {
            progress += 1;
            if progress % (u16::MAX / 100) == 0 {
                progress_bar.inc();
            }
            ports.push(port);
        }

        // print result summary
        let n = ports.len();
        match n {
            0 => progress_bar.print_final_info(
                "Result:",
                &format!("{n} open ports found."),
                Color::Red,
                Style::Bold,
            ),
            1 => progress_bar.print_final_info(
                "Result:",
                &format!("{n} open port found:"),
                Color::Green,
                Style::Bold,
            ),
            _ => progress_bar.print_final_info(
                "Result:",
                &format!("{n} open ports found:"),
                Color::Green,
                Style::Bold,
            ),
        };

        // print successfully connected ports
        ports.sort_unstable();
        let mut n = 0;
        for port in ports {
            n += 1;
            progress_bar.print_info(
                &format!("{n}."),
                &format!("{port}"),
                Color::Blue,
                Style::Normal,
            );
        }
    }
}
