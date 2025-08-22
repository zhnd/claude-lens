use clap::Parser;
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, warn};

mod config;
mod server;
mod api;
mod otel;
mod storage;

use config::Config;

#[derive(Parser, Debug)]
#[command(name = "claude-scope")]
#[command(about = "Claude Code monitoring tool with OpenTelemetry data collection")]
struct Args {
    #[arg(long, default_value = "3000")]
    port: u16,

    #[arg(long, default_value = "4317")]
    otel_port: u16,

    #[arg(long, default_value = "./claude-scope.db")]
    db_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("claude_scope=info,tower_http=debug"))
        )
        .init();

    let args = Args::parse();
    
    // Create configuration
    let mut config = Config::from_env();
    config.http_port = args.port;
    config.otel_port = args.otel_port;
    config.database_path = args.db_path.clone();

    info!("Starting Claude Scope");
    info!("HTTP server will listen on port {}", config.http_port);
    info!("OpenTelemetry gRPC server will listen on port {}", config.otel_port);
    info!("Database path: {}", config.database_path);

    // Initialize database
    let db = storage::sqlite::init_database(&config.database_path).await?;
    info!("Database initialized");

    // Start both servers concurrently
    let http_addr: SocketAddr = ([0, 0, 0, 0], config.http_port).into();
    let otel_addr: SocketAddr = ([0, 0, 0, 0], config.otel_port).into();

    let http_server = server::start_http_server(http_addr, db.clone());
    let otel_server = otel::receiver::start_otel_server(otel_addr, db.clone());

    tokio::select! {
        result = http_server => {
            if let Err(e) = result {
                warn!("HTTP server error: {}", e);
            }
        }
        result = otel_server => {
            if let Err(e) = result {
                warn!("OpenTelemetry server error: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully...");
        }
    }

    info!("Claude Scope shutdown complete");
    Ok(())
}