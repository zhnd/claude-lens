use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer},
    trace::TraceLayer,
    services::ServeDir,
};
use tracing::{info, warn};

use crate::{api, storage::Database};

pub async fn start_http_server(
    addr: SocketAddr,
    db: Arc<dyn Database>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app(db).await;

    info!("HTTP server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn create_app(db: Arc<dyn Database>) -> Router {
    // API routes with database state
    let api_routes = api::create_routes().with_state(db);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    // Create static file service for the entire web/dist directory
    let static_service = ServeDir::new("web/dist")
        .append_index_html_on_directories(true);

    Router::new()
        .nest("/api", api_routes)
        .route("/", get(serve_index))
        // Serve all static files from web/dist, excluding API routes
        .fallback_service(static_service)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        )
}

async fn serve_index() -> impl IntoResponse {
    // Check if frontend build exists
    if std::path::Path::new("web/dist/index.html").exists() {
        // Read and serve the built index.html
        match tokio::fs::read_to_string("web/dist/index.html").await {
            Ok(content) => Html(content),
            Err(_) => {
                warn!("Failed to read built index.html, serving fallback");
                serve_fallback_html()
            }
        }
    } else {
        // Fallback to basic HTML if frontend build is not available
        warn!("Frontend build not found, serving fallback HTML");
        serve_fallback_html()
    }
}

fn serve_fallback_html() -> Html<String> {
    Html(r#"<!DOCTYPE html>
<html>
<head>
    <title>Claude Scope - Monitoring Dashboard</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body { font-family: system-ui, sans-serif; max-width: 800px; margin: 0 auto; padding: 2rem; }
        .warning { background: #fff3cd; border: 1px solid #ffeaa7; padding: 1rem; border-radius: 0.5rem; margin: 1rem 0; }
        ul { line-height: 1.6; }
        a { color: #0066cc; }
    </style>
</head>
<body>
    <h1>🔭 Claude Scope</h1>
    <div class="warning">
        <strong>⚠️ Development Mode:</strong> Frontend build not found. 
        <br>Run <code>cd web && npm install && npm run build</code> to build the frontend.
    </div>
    <p>Claude Code monitoring tool is running!</p>
    <h2>API Endpoints:</h2>
    <ul>
        <li><a href="/api/health">Health Check</a></li>
        <li><a href="/api/metrics/overview">Metrics Overview</a></li>
        <li><a href="/api/metrics/timeline?range=24h">Timeline (24h)</a></li>
        <li><a href="/api/sessions">Sessions</a></li>
    </ul>
    <p><em>Frontend dashboard will be available after building the web assets.</em></p>
</body>
</html>"#.to_string())
}

async fn serve_fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "File not found")
}