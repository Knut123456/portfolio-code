use std::process::Command;

fn main() {
    // is a powershelll command that ask the virtual machine to give me is ip address
    const PATH: &str = r"src\config\vm_config.json";
    let file = read_to_string(PATH)
        .expect("Unable to read file");

    let json: Value = from_str(&file)
        .expect("JSON does not have correct format.");

    let json_vm = &json["database"];
    let vm_name = &json_vm["name"];

    let ps_script = format!(r#"
    Get-VMNetworkAdapter -VMName "{}" |
        Select-Object -ExpandProperty IPAddresses
    "#, vm_name);

    let output =Command::new("powershell")
        .args(&["-Command", ps_script])
        .output()
        .expect("failed to excute powershell script");

    if output.status.success(){
            // Convert and print the standard output
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Output:\n{}", stdout);
        } else {
            // Convert and print the standard error
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error:\n{}", stderr);
        }  
}