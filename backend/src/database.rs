use mysql::*;
use mysql::prelude::*;
use std::fs::read_to_string;
use serde_json::{ Value, from_str};
pub fn database() -> Result<()> {
    let file = read_to_string("config/config.json")
        .expect("Unable to read file");

    let json: Value = from_str(&file)
        .expect("JSON does not have correct format.");
    let json_database = &json["database"];
    let ipaddress = &json_database["ipaddress"];
    let user = &json_database["user"];
    let port = &json_database["port"];
    let password = &json_database["password"];
    
   
    // Define the database URL
    let url: String = format!("mysql://{user}:{password}@{ipaddress}:{port}");
    let convert_str: &str = url.as_str();
    // Create a connection pool
    let pool = Pool::new(convert_str)?;
   
    // Get a connection from the pool
    let mut conn = pool.get_conn()?;

    // Execute a query
    let result: Vec<(i32, String)> = conn.query("SELECT id, name FROM users")?;

    // Process the results
    for (id, name) in result {
        println!("ID: {}, Name: {}", id, name);
    }

    Ok(())
}
