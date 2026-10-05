mod cli;
mod config;
mod db;
mod routes;
mod state;

use clap::Parser;
use cli::Cli;
use config::AppConfig;
use state::AppState;
use std::process;
use std::sync::Arc;

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

    let pool = match db::init_db(&config.database).await {
        Ok(p) => {
            println!("Database connection pool initialized and verified successfully.");
            p
        }
        Err(e) => {
            eprintln!("Database initialization failed: {}", e);
            process::exit(1);
        }
    };

    let host = config.server_host.clone();
    let port = config.server_port;

    let app_state = AppState {
        pool,
        config: Arc::new(config),
    };

    let app = routes::create_router().with_state(app_state);

    let address = format!("{}:{}", host, port);
    let listener = match tokio::net::TcpListener::bind(&address).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", address, e);
            process::exit(1);
        }
    };

    println!("Application started successfully. Listening on {}", address);

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
        process::exit(1);
    }
}
