
use std::sync::Arc;
use axum::{
    routing::{get_service, get},
    Router,
    response::IntoResponse,
    Json, 
    extract::State
};
use std::error::Error;
use std::result::Result;
use tower_http::services::{ServeDir, ServeFile};
mod database;
use tokio; 
use serde::Serialize;
use serde_json::{Value, to_value, Result as jsonresult};


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
#[derive(Clone)]
struct AppState {
    pool: sqlx::MySqlPool,
}
async fn app() -> Router {
    let pool = database::database_func()
        .await
        .expect("failed to connect to database");
    let static_files = ServeDir::new("src/assets/assets");
    let static_html = ServeFile::new("src/assets/index.html");
    let app = Router::new()
        .route_service("/", get_service(static_html))
        .nest_service("/assets", static_files)
        .route("/project", get(project))
        .with_state(Arc::new(AppState { pool }))
        ;   
      
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
async fn project(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Acquire a connection—acquire() 
    let mut conn = state.pool.acquire().await 
        .expect("DB connection failed");
    // Run your query
    let projects: Vec<Project> = sqlx::query_as!(
        Project, 
        "SELECT id, name, img_url, link_url, description FROM projects"
    )
    .fetch_all(&mut conn)
    .await
    .expect("Query failed");
    // Return JSON; 
    Json(projects)
}

