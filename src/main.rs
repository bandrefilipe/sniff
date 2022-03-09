use std::{env, process};
use ip_sniffer::run_application;

fn main() {
    let args: Vec<String> = env::args().collect();
    process::exit( match run_application(&args) {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
