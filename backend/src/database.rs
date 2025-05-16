use mysql::*;
use std::fs::read_to_string;
use serde_json::{Value, from_str};


pub fn database_func() -> Result<PooledConn> {
    const PATH: &str = r"src\config\config.json";
    let file = read_to_string(PATH)
        .expect("Unable to read file");

    let json: Value = from_str(&file)
        .expect("JSON does not have correct format.");

    let json_database = &json["database"];

    let ipaddress = &json_database["ipaddress"];
    let user = &json_database["user"];
    let port = &json_database["port"];
    let password = &json_database["password"];
    let host = &json_database["host"];
    
    // Define the database URL
    let url: String = format!("mysql://{user}:{password}@{ipaddress}:{port}/{host}");
    let convert_str: &str = url.as_str();

    // Create a connection pool
    let pool = Pool::new(convert_str)?;

    // Get a connection from the pool
    let conn = pool.get_conn()?;

    Ok(conn)
}
