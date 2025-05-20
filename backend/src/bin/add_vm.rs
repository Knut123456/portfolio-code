use std::io;
use std::fs::{File, remove_file};
use std::io::{Result, Write};
use serde_json::json;
use std::path::Path;


const PATH: &str = r"src\config\vm_config.json";

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
        let vm = read_input("Do you use VM: (yes or no) ");
        let name = read_input("What is the name of the VM:  ");
        let config = json!({
            "VM": {
                "VM": vm,
                "name": name,
                
            }
        });

    // Step 2: Open or create the file
    let mut file = File::create(PATH)?;

    // Step 3: Write the JSON object to the file
    write!(file, "{}", config.to_string())?;
    
    
} 
    Ok(())
    
}

