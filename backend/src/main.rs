
use axum::{
    routing::{get_service, get},
    Router,
    response::IntoResponse,
    Json, 
};
use tower_http::services::{ServeDir, ServeFile};
use tokio::{self}; 
use serde::Serialize;
use sqlx::{mysql, prelude::FromRow, Connection};
use dotenvy;
mod database;
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]

async fn main() {
    let app: Router = app().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("server on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app)
        .await
        .unwrap();

}


async fn app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);
    let static_files = ServeDir::new("src/assets/assets");
    let static_html = ServeFile::new("src/assets/index.html");
    let app = Router::new()
        .route_service("/", get_service(static_html))
        .nest_service("/assets", static_files)
        .route("/project", get(project))
        .layer(cors)
        ;
        
      
    app  // returned implicitly
}
#[derive(Debug, FromRow, Serialize )]
struct ProjectStruct {
    id: i32,
    name: String, 
    img_url: String,
    link_url: String, 
    description: String 
}

 async fn project(
    
 ) -> impl IntoResponse {
    
      dotenvy::dotenv().ok();                       

    
    
    let opt = database::database_func().await;
    let mut connection = mysql::MySqlConnection::connect_with(&opt).await.unwrap();

    let rows: Vec<ProjectStruct> = sqlx::query_as("SELECT * FROM projects").fetch_all(&mut connection).await.unwrap();

    
    match connection.close().await {
        Ok(()) => {
            println!("Connection closed successfully.");
        }
        Err(e) => {
            eprintln!("Failed to close connection: {}", e);
            // You could perform additional recovery or logging here.
        }
    }

    Json(rows)
 }