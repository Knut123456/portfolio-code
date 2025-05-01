use std::io;
use indexmap::IndexMap;
use dialoguer::MultiSelect;
use std::fs::File;
use std::io::{Result, Write};
use serde_json::{json, Value};

const PATH: &str = r"src\config\config.json";

fn create_toml() -> Result<()>{
// Start with an empty document
let config = json!({
    "name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
});

// Step 2: Open or create the file
let mut file = File::create(PATH)?;

// Step 3: Write the JSON object to the file
write!(file, "{}", config.to_string())?;
Ok(())

}  
fn main() {
    match create_toml() {
        Ok(value) => {
            // Handle success
            println!("Success: {:?}", value);
        }
        Err(e) => {
            // Handle error
            eprintln!("Error: {:?}", e);
        }
    }
    let items = &["Apple", "Banana", "Cherry"];
    let selection = MultiSelect::new()
        .with_prompt("Pick your favorite fruits")
        .items(items)
        .interact()
        .unwrap(); // :contentReference[oaicite:13]{index=13}

    for idx in selection {
        println!("You chose: {}", items[idx]);
    }  
    
    let mut input_map  = IndexMap::new();
    input_map.insert("database settings", database_config as fn());
    input_map.insert("connect vm", connect_to_vm_paramters as fn());
    input_map.insert("exit ", exit as fn());
    
    
    loop {
        print!("please enter a valid number: 

");
        let mut input_map_count = -1;
        // Loop through the input_map and print each key-value pair
        for (key, _value) in &input_map {
            input_map_count += 1;
            println!("{}: {}", input_map_count, key );
        }
    
        // Flush the output buffer to ensure the prompt is displayed immediately
        let mut input: String = String::new(); // Create a string variable
        io::stdin() // Get the standard input stream
            .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
            .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

        match input.trim().parse::<usize>() { // Parse the input string to a usize
            Ok(index) if index < input_map.len() => { // Check if the index is within bounds
                // Retrieve and call the corresponding function
                if let Some((key, function)) = input_map.get_index(index) { // Get the key and function at the specified index
                    // Call the function
                    println!("You selected: {}", key);
                    function();
                    break;
                }
            }
            _ => println!("Invalid selection. Please enter a valid number."),
            
        
        }
    }       

}

fn exit() {
    // Add logic for exiting the program here
    println!("Exiting the program...");
    std::process::exit(0); // Exit the program with a success status code
}

fn database_config() {

}

fn connect_to_vm_paramters(){
    println!("write the name of the virtual machine your are using: ");
    let mut vm_machine_name: String = String::new(); // Create a string variable
    io::stdin() // Get the standard input stream
        .read_line(&mut vm_machine_name) // The read_line function reads data until it reaches a '\n' character
        .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message
    /* let string_slice: &str = vm_machine_name.as_str(); */
    
}


