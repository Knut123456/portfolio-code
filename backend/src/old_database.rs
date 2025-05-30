use sqlx::{
    Connection,
    mysql::{MySqlPoolOptions, MySqlConnection, },
    MySqlPool, pool::PoolConnection,
};
use serde::Deserialize;
use std::{error::Error, fs, path::Path};

#[derive(Deserialize)]
struct DatabaseConfig {
    ipaddress: String,
    user: String,
    port: Option<u16>,
    password: String,
    database: String,
}

#[derive(Deserialize)]
struct Config {
    database: DatabaseConfig,
}

pub async fn database_func() -> Result<MySqlPool, Box<dyn Error>> {

    let contents = fs::read_to_string(Path::new("src/config/config.json"))?;
    let cfg: Config = serde_json::from_str(&contents)?;
    let db = cfg.database;


    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.ipaddress, db.port.unwrap_or(3306), db.database
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;  

    //ping the server
    let mut conn: PoolConnection<sqlx::MySql> = pool.acquire().await?;
    conn.ping().await?;

    Ok(pool)
}