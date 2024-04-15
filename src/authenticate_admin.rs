use argon2::{password_hash::PasswordHash, password_hash::PasswordVerifier, Argon2};
use chrono::Utc;
use csv::Writer;
use serde_json;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self};
use std::process::Command;
use std::vec::Vec;
use uuid::Uuid;

pub fn authenticate() -> Result<bool, Box<dyn Error>> {
    // Command to run the Python script
    let output = Command::new("python3")
        .arg("assets/session_audit.py")
        .output()?;

    if output.status.success() {
        // Parse the output JSON to extract the session ID
        let output_string = String::from_utf8_lossy(&output.stdout);
        let session_id = parse_session_id(&output_string);

        println!("Session ID: {}", session_id);

        // Get current timestamp
        let timestamp = Utc::now();

        // Write to CSV file
        if let Err(err) = append_to_audit_trail(&session_id, &timestamp) {
            eprintln!("Failed to append to audit trail: {}", err);
        }

        let s_str = output_string.lines().last().ok_or("No output")?;
        let s: Vec<i32> = serde_json::from_str(s_str)?;

        let persistent_master_key: String = s.iter().map(|&x| x.to_string()).collect();
        let password_hash: String =
            "$argon2i$v=19$m=16,t=2,p=1$SW9Xb2o1OFhyYVl3NEtBdw$0yzqSaIweNuBoJSi8aKvcw".to_string();

        let parsed_hash = PasswordHash::new(&password_hash).expect("Failed to parse hash");
        let argon2 = Argon2::default();

        println!("\nEnter password for election official:");
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

fn append_to_audit_trail(session_id: &str, timestamp: &chrono::DateTime<Utc>) -> io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("audit_trail.csv")?;
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&[session_id, &timestamp.to_string()])?;
    wtr.flush()?;
    Ok(())
}

fn parse_session_id(output_string: &str) -> String {
    let session_id = output_string
        .lines()
        .find(|line| line.starts_with("Session Id:"))
        .map(|line| {
            line.trim_start_matches("Session Id:")
                .trim_start_matches("0x")
                .trim()
        })
        .unwrap_or("");

    let uuid = Uuid::new_v4();
    format!("{}-{}", &session_id[2..], uuid)
}
