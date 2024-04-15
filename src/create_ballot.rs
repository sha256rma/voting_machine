use crate::models::{Candidate, Election, Office};
use crate::save_ballot::save_to_csv;
use chrono::Utc;
use log::{error, info};
use std::error::Error;
use std::io::{self};
use uuid::Uuid;

fn create_election_ballot() -> Result<(), Box<dyn Error>> {
    // Log the start of the function
    info!("Creating a new election ballot...");

    println!("Creating a new election ballot...");

    // Get election name from user input
    println!("Enter the election name:");
    let mut election_name = String::new();
    io::stdin()
        .read_line(&mut election_name)
        .expect("Failed to read line");
    let election_name = election_name.trim().to_string();
    let election_id = Uuid::new_v4();
    let is_open = true;

    let election = Election {
        election_id,
        name: election_name.clone(),
        is_open,
    };

    // Log election details
    let timestamp = Utc::now();
    info!("{}: Election created: {:?}", timestamp, election);

    // Saving election
    let election_headers = ["election_id", "name", "is_open"];
    if let Err(err) = save_to_csv(&[election], &election_headers, "elections.csv") {
        // Log error if saving fails
        let timestamp = Utc::now();
        error!("{}: Failed to save elections to CSV: {}", timestamp, err);
        // Handle the error appropriately, maybe return it
        return Err(err);
    }

    // Initialize offices
    let mut offices = Vec::new();

    loop {
        // Get office name from user input
        println!("Enter the office name (or type 'done' to finish adding offices):");
        let mut office_name = String::new();
        io::stdin()
            .read_line(&mut office_name)
            .expect("Failed to read line");
        let office_name = office_name.trim().to_string();

        if office_name == "done" {
            break;
        }
        let office_id = Uuid::new_v4();
        let mut candidates = Vec::new();

        loop {
            // Get candidate name and political party from user input
            println!("Enter the candidate name for office '{}':", office_name);
            let mut candidate_name = String::new();
            let candidate_id = Uuid::new_v4();

            io::stdin()
                .read_line(&mut candidate_name)
                .expect("Failed to read line");
            let candidate_name = candidate_name.trim().to_string();

            println!(
                "Enter the political party for candidate '{}':",
                candidate_name
            );
            let mut political_party = String::new();
            io::stdin()
                .read_line(&mut political_party)
                .expect("Failed to read line");
            let political_party = political_party.trim().to_string();

            let candidate = Candidate {
                candidate_id,
                name: candidate_name.clone(),
                office_id,
                political_party,
                election_id
            };
            candidates.push(candidate.clone()); // Clone the candidate and push it into the vector

            // Log candidate details
            let timestamp = Utc::now();
            info!(
                "{}: Candidate added for office '{}': {:?}",
                timestamp, office_name, candidate
            );

            println!(
                "Do you want to add another candidate for office '{}'? (yes/no)",
                office_name
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim().to_lowercase();
            if input != "yes" {
                // Saving candidates
                let candidates_headers = [
                    "candidate_id",
                    "name",
                    "office_id",
                    "political_party",
                    "election_id"
                ];
                if let Err(err) = save_to_csv(&candidates, &candidates_headers, "candidates.csv") {
                    // Log error if saving fails
                    let timestamp = Utc::now();
                    error!("{}: Failed to save candidates to CSV: {}", timestamp, err);
                    // Handle the error appropriately, maybe return it
                    return Err(err);
                }
                break;
            }
        }

        let office = Office {
            office_id,
            office_name: office_name.clone(),
            election_id,
            election_name: election_name.clone(),
        };
        offices.push(office.clone()); // Clone the office and push it into the vector

        // Log office details
        let timestamp = Utc::now();
        info!("{}: Office added: {:?}", timestamp, office);
    }

    // Saving offices
    let offices_headers = ["office_id", "office_name", "election_id", "election_name"];
    if let Err(err) = save_to_csv(&offices, &offices_headers, "offices.csv") {
        // Log error if saving fails
        let timestamp = Utc::now();
        error!("{}: Failed to save offices to CSV: {}", timestamp, err);
        // Handle the error appropriately, maybe return it
        return Err(err);
    }

    // Log the success of the operation
    let timestamp = Utc::now();
    info!("{}: Election ballot creation successful.", timestamp);

    Ok(())
}

// Public function exposed by this module
pub fn public_interface_to_create_ballot() -> Result<(), Box<dyn Error>> {
    match create_election_ballot() {
        Ok(_) => {
            println!("Election ballot creation successful.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error occurred while creating election ballot: {}", e);
            Err(e)
        }
    }
}
