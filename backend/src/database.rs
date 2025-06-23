use sqlx::{mysql, Connection};
use dotenvy;
use std::env;

pub async fn database_func() -> mysql::MySqlConnectOptions {

    dotenvy::dotenv().ok();     
    let ip = env::var("ip")
        .expect("ip must be set in .env or environment");
    let port = env::var("port")
        .expect("port must be set in .env or environment");
    let bruker = env::var("bruker")
        .expect("bruker must be set in .env or environment");
    let passord = env::var("passord")
        .expect("passord must be set in .env or environment");
    let database = env::var("database")
        .expect("database must be set in .env or environment");
    let ip: &str = ip.as_str();
    let port: u16 = port
        .parse::<u16>()             
        .expect("port must be a valid u16");
    let bruker: &str = bruker.as_str(); 
    let passord: &str = passord.as_str(); 
    let database: &str = database.as_str(); 
    let opt = mysql::MySqlConnectOptions::new().host(ip).port(port).username(bruker).password(passord).database(database);
    let connection = mysql::MySqlConnection::connect_with(&opt).await.unwrap();

    match connection.close().await {
        Ok(()) => {
            println!("Connection closed successfully.");
        }
        Err(e) => {
            eprintln!("Failed to close connection: {}", e);
        }
    }
    opt
}