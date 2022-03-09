use ip_sniffer::run_application;
use std::process;

fn main() {
    process::exit(match run_application() {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("error: {error}");
            1
        }
    });
}
