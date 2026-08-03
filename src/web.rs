mod api;
mod models;

use axum::{
    routing::{get, post},
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

        // Serve static files
        .nest_service("/", ServeDir::new("static"));

    // Render sets the PORT environment variable
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 TextDistance-RS running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
