mod api;
mod models;

use axum::{
    routing::post,
    Router,
};

use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {

    // Build the application
    let app = Router::new()

        // API endpoint
        .route("/api/calculate", post(api::calculate))

        // Serve frontend
        .fallback_service(ServeDir::new("static"));

    // Render automatically sets PORT.
    // Local development defaults to 3000.
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 TextDistance-RS running on http://{}", addr);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    // Start server
    axum::serve(listener, app)
        .await
        .unwrap();
}
