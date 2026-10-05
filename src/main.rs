mod cli;
mod config;
mod db;

use clap::Parser;
use cli::Cli;
use config::AppConfig;
use std::process;

#[tokio::main]
async fn main() {
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

    println!("Configuration loaded successfully.");

    let _pool = match db::init_db(&config.database).await {
        Ok(p) => {
            println!("Database connection pool initialized and verified successfully.");
            p
        }
        Err(e) => {
            eprintln!("Database initialization failed: {}", e);
            process::exit(1);
        }
    };

    // Application logic will follow here in future specs
    println!("Application started successfully.");
}
