use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::error::Error;
use serde_json::{Value, from_str};
use std::fs:: read_to_string;
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
    // Create the pool
    let url: String = format!("mysql://{user}:{password}@{ipaddress}:{port}");
    let convert_str: &str = url.as_str();
    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .connect(convert_str)
        .await?;

    // SQL statement(s) from a file
    let sql = include_str!("database.sql");

    // Use `&pool` which *does* implement `Executor`
    sqlx::query(sql)
        .execute(&pool) // ✅ This works
        .await?;

    println!("SQL executed successfully.");
    Ok(())
}