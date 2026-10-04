mod cli;
mod config;

use clap::Parser;
use cli::Cli;
use config::AppConfig;
use std::process;

fn main() {
    let cli = Cli::parse();

    let config = match AppConfig::load(cli.config.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = config.validate() {
        eprintln!("Configuration validation failed: {}", e);
        process::exit(1);
    }

    println!("Configuration loaded successfully: {:?}", config);
}
