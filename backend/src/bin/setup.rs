use std::io;
use indexmap::IndexMap;
use std::collections::HashMap;
use serde_json;
use std::fs::File;
use std::io::{Write, Result};

fn create_json() -> Result<()>{
    let mut database_connect = HashMap::new();
    database_connect.insert("user", "");
    database_connect.insert("password", "");
    database_connect.insert("host", "");
    database_connect.insert("database", "");
    let mut vm: HashMap<&str, &str> = HashMap::new();
        vm.insert("VM", "0");
        vm.insert("VM machine", "");
    let mut json_config = HashMap::new();
    json_config.insert("VM", vm);
    json_config.insert("Connect_to_database", database_connect);
    let json = serde_json::to_string_pretty(&json_config).unwrap();
    // Print JSON to console
    println!("{}", json);

    // Write to file
    let mut file = File::create("backend/src/config/connect_to_database.json")?;
    file.write_all(json.as_bytes())?;

    println!("Data successfully written to connect_to_database.json");
    
    Ok(())
}  
fn main() {
    let mut create_jsons = create_json();
    Ok((create_jsons));
    
    let mut input_map  = IndexMap::new();
    input_map.insert("database settings", database_config as fn());
    input_map.insert("connect ubuntu", connect_to_vm_paramters as fn());
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

fn database_config(){
    let mut data_input_map  = IndexMap::new();
    data_input_map.insert("exit ", exit as fn());
    
    loop {
        print!("please enter a valid number: ");
        print!("

");
        let mut input_map_count = -1;
        // Loop through the input_map and print each key-value pair
        for (key, _value) in &data_input_map {
            input_map_count += 1;
            println!("{}: {}", input_map_count, key );
        }
    
        // Flush the output buffer to ensure the prompt is displayed immediately
        let mut database_input: String = String::new(); // Create a string variable
        io::stdin() // Get the standard input stream
            .read_line(&mut database_input) // The read_line function reads data until it reaches a '\n' character
            .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

        match database_input.trim().parse::<usize>() { // Parse the input string to a usize
            Ok(index) if index < data_input_map.len() => { // Check if the index is within bounds
                // Retrieve and call the corresponding function
                if let Some((key, function)) = data_input_map.get_index(index) { // Get the key and function at the specified index
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

fn connect_to_vm_paramters(){
    println!("write the name of the virtual machine your are using: ");
    let mut vm_machine_name: String = String::new(); // Create a string variable
    io::stdin() // Get the standard input stream
        .read_line(&mut vm_machine_name) // The read_line function reads data until it reaches a '\n' character
        .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message
    /* let string_slice: &str = vm_machine_name.as_str(); */
    
}


