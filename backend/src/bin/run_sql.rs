/* 
todo make it write and create a database

use sqlx:: Pool;
use std::error::Error;
use serde_json::{Value, from_str};
use std::fs:: read_to_string;
use mysql::*;
const PATH: &str = r"src\config\config.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string(PATH)
        .expect("Unable to read file");

    let json: Value = from_str(&file)
        .expect("JSON does not have correct format.");

    let json_database = &json["database"];
    let ipaddress = &json_database["ipaddress"];
    let user = &json_database["user"];
    let port = &json_database["port"];
    let password = &json_database["password"];
    let url: String = format!("mysql://{user}:{password}@{ipaddress}:{port}");
    
    // Create the pool
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // SQL statement(s) from a file
    let sql = include_str!("database.sql");

    // Use `&pool` which *does* implement `Executor`
    sqlx::query(sql)
        .execute(&pool) // ✅ This works
        .await?;

    println!("SQL executed successfully.");
    Ok(())
} */