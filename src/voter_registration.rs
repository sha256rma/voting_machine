use crate::models::Voter;
use csv::{ReaderBuilder, Trim, Writer};
use log::{error, info};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self};
use std::path::Path;

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
        wtr.write_record(&[
            "user_id",
            "national_id",
            "name",
            "date_of_birth",
            "has_voted",
        ])?;
    }

    // Manually write the voter information to the CSV
    wtr.write_record(&[
        voter.user_id.to_string(),
        voter.national_id.clone(),
        voter.name.clone(),
        voter.date_of_birth.clone(),
        voter.has_voted.to_string(),
    ])?;

    wtr.flush()?;
    info!("Voter {} successfully registered.", voter.clone().name);
    Ok(())
}

fn validate_date_of_birth(dob: &str) -> Result<(), String> {
    let regex = Regex::new(r"^(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])-\d{4}$").unwrap();
    if regex.is_match(dob) {
        Ok(())
    } else {
        Err("Invalid date of birth format.".into())
    }
}

fn validate_national_id(id_number: &str) -> Result<(), String> {
    let regex = Regex::new(r"^\d{3}-\d{3}-\d{3}$").unwrap();
    if regex.is_match(id_number) {
        Ok(())
    } else {
        Err("Invalid ID number format.".into())
    }
}

fn validate_name(name: &str) -> Result<(), String> {
    let regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
    if regex.is_match(name) {
        Ok(())
    } else {
        Err("Invalid name format.".into())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserRegistration {
    user_id: String,
    name: String,
    date_of_birth: String,
    national_id: String,
    has_registered: bool,
    has_voted: bool,
}

pub fn interactively_register_voter() -> Result<(), Box<dyn Error>> {
    info!("Starting voter registration process...");

    println!("Registering a new voter...");

    let mut national_id = String::new();
    // Loop for ID number input
    loop {
        println!("Enter personal ID (xxx-xxx-xxx):");
        national_id.clear(); // Clear previous input
        io::stdin().read_line(&mut national_id)?;
        national_id = national_id.trim().to_string();

        if validate_national_id(&national_id).is_ok() {
            break; // Exit loop if valid
        } else {
            println!("Invalid ID number format. Expected format: xxx-xxx-xxx, where x is a number from 0-9.");
        }
    }
    let mut name = String::new();
    // Loop for name input
    loop {
        println!("Enter your name to confirm:");
        name.clear(); // Clear previous input
        io::stdin().read_line(&mut name)?;
        name = name.trim().to_string();

        if validate_name(&name).is_ok() {
            break; // Exit loop if valid
        } else {
            println!("Invalid name format. Name should only contain letters and spaces.");
        }
    }

    let mut date_of_birth = String::new();
    loop {
        println!("Enter date of birth (MM-DD-YYYY):");
        date_of_birth.clear(); // Clear previous input
        io::stdin().read_line(&mut date_of_birth)?;
        date_of_birth = date_of_birth.trim().to_string();

        if validate_date_of_birth(&date_of_birth).is_ok() {
            break; // Exit loop if valid
        } else {
            println!("Invalid date of birth format. Please use MM-DD-YYYY format.");
        }
    }

    info!("User input validated successfully.");

    // Read users.csv and try to find the user by national_id
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All) // Trim leading and trailing whitespace from all fields
        .from_path("users.csv")?;

    let mut user_confirmed = false;
    let mut found_user: Option<UserRegistration> = None;
    for result in rdr.deserialize::<UserRegistration>() {
        let record: UserRegistration = result?;
        if record.national_id == national_id
            && record.name == name
            && record.date_of_birth == date_of_birth
        {
            if record.has_registered || record.has_voted {
                error!("User {} has already registered or voted.", name);
                return Err("User has already registered or voted.".into());
            }
            found_user = Some(record);
            user_confirmed = true;
            break;
        }
    }
    let mut user = match found_user {
        Some(user) => user,
        None => {
            error!("User not found.");
            return Err("User not found.".into());
        }
    };

    if user_confirmed {
        info!("User details confirmed. Proceeding with registration...");
        // Proceed to update the user as registered
        user.has_registered = true;
        update_user_registration(&user)?;
        info!("Voter successfully registered.");
    } else {
        error!("The details do not match our records.");
        return Err("User details incorrect.".into());
    }

    let voter = Voter::new(user.user_id, national_id, user.name, date_of_birth);
    register_voter(voter)?;

    Ok(())
}

fn update_user_registration(updated_user: &UserRegistration) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All) // Trim leading and trailing whitespace from all fields
        .from_path("users.csv")?;
    let mut records: Vec<UserRegistration> = rdr.deserialize().map(|r| r.unwrap()).collect();

    // Find the record to update
    let pos = records
        .iter()
        .position(|r| r.national_id == updated_user.national_id)
        .unwrap();
    records[pos] = updated_user.clone();

    // Write all records back to the file, including the updated one
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("users.csv")?;
    let mut wtr = Writer::from_writer(file);
    for record in records {
        wtr.serialize(&record)?;
    }
    wtr.flush()?;
    Ok(())
}
