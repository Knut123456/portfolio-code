use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
mod database;


#[tokio::main]
async fn main() {
    // Create a router with multiple routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/project", get(project));
        

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Welcome to the API"
}

async fn project() -> &'static str {
    "List of users"
}

