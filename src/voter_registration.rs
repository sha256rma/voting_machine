use std::error::Error;
use std::fs::OpenOptions;
use csv::Writer;
use crate::models::Voter;
use std::path::Path;
use std::io::{self};


pub fn register_voter(voter: Voter) -> Result<(), Box<dyn Error>> {
    let file_path = "registered_voters.csv";
    let file_exists = Path::new(file_path).exists();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut wtr = Writer::from_writer(file);

    // If the file did not previously exist, write the header
    if !file_exists {
        wtr.write_record(&["user_id", "name", "date_of_birth", "has_voted"])?;
    }

    // Manually write the voter information to the CSV
    wtr.write_record(&[
        voter.user_id.to_string(),
        voter.name,
        voter.date_of_birth,
        voter.has_voted.to_string(),
    ])?;

    wtr.flush()?;
    Ok(())
}

pub fn interactively_register_voter() -> Result<(), Box<dyn Error>> {
    println!("Registering a new voter...");

    println!("Enter user ID:");
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id)?;
    let user_id: u64 = user_id.trim().parse()?;

    println!("Enter name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    println!("Enter date of birth (YYYY-MM-DD):");
    let mut date_of_birth = String::new();
    io::stdin().read_line(&mut date_of_birth)?;
    let date_of_birth = date_of_birth.trim().to_string();

    let voter = Voter::new(user_id, name, date_of_birth);

    register_voter(voter)?;

    println!("Voter successfully registered.");
    Ok(())
}
