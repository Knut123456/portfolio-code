use std::process::Command;

fn main() {
    let ps_script =r#"
         Get-VMNetworkAdapter -VMName "ubuntu" | Select-Object -ExpandProperty IPAddresses
    "#;

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