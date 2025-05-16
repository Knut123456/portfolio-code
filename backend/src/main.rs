
use axum;
use axum::routing::{get_service, Route};
use axum::{
    routing::get,
    Router,
};
use mysql::{Pool, prelude::*};
use std::result::Result;
use tower_http::services::{ServeDir, ServeFile};
mod database;
use tokio; 
use serde::Serialize;


#[tokio::main]

async fn main() {
    let app: Router = app();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("server on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app)
        .await
        .unwrap();

}

fn app() -> Router {
    let static_files = ServeDir::new("src/assets/assets");
    let static_html = ServeFile::new("src/assets/index.html");
    let project = project();
    let app = Router::new()
        .route_service("/", get_service(static_html))
        .nest_service("/assets", static_files)
        .route("/project", get(project));     
    app  // returned implicitly
}

#[derive(Debug, Serialize)]
struct Project {
    id: i32,
    name: String, 
    img_url: String,
    link_url: String, 
    description: String 
}
fn project() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = database::database_func()?;
    let project = conn.query_map(
        "SELECT id, name, img_url, link_url, description FROM projects",
        |(id, name, img_url, link_url, description)| Project { id, name, img_url, link_url, description },
    )?;
      // 2a. Compact JSON string
    let json_compact = serde_json::to_string(&project)?;
    Ok(json_compact);
   
}

