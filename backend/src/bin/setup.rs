use std::io;
use std::fs::{File, remove_file};
use std::io::{Result, Write};
use serde_json::json;
use std::path::Path;
use std::net::IpAddr;

const PATH: &str = r"src\config\config.json";

fn main() -> Result<()> {
    // Start with an empty document
    match remove_file(PATH) {
        Ok(_) => println!("File '{}' has been removed.", PATH),
        Err(e) => eprintln!("Failed to remove file '{}': {}", PATH, e),
    }
    if !Path::new(PATH).exists() {
         // Helper function to read input from the user
        fn read_input(prompt: &str) -> String {
            print!("{}", prompt);
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string() // converts it into a string
        }

        // Prompt the user for each field
        let ipaddressstring = read_input("Enter IP address: ");
        let ipaddress: IpAddr = ipaddressstring.parse().expect("Not a valid IP address");
        let user: String = read_input("Enter username: ");
        let password = read_input("Enter password: ");
        let portstring = read_input("Enter port (default 3306): ");
        let port: i32 = portstring.parse().expect("Not a valid number");
        let database = "PORTFOLIO".to_string();
        let config = json!({
            "database": {
                "ipaddress": ipaddress,
                "user": user,
                "password": password,
                "port": port,
                "database": database
            }
        });

    //  Open or create the file
    let mut file = File::create(PATH)?;

    //  Write the JSON object to the file
    write!(file, "{}", config.to_string())?;
    
    
} 
    Ok(())
    
}

