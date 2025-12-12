mod errors;
mod models;
mod storage;
mod service;
mod cli;

use std::process;

fn main() {
    // Use current directory for data storage
    let data_dir = ".";
    
    if let Err(e) = cli::run(data_dir) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
