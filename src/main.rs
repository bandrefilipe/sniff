mod application;
mod notification;
mod scan;

use crate::application::run_application;
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
