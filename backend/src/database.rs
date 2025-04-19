use mysql::*;
use mysql::prelude::*;

pub fn database() -> Result<()> {
    
    // Define the database URL
    let url = "mysql://username:password@localhost:3306/database_name";

    // Create a connection pool
    let pool = Pool::new(url)?;

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
