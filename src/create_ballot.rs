use crate::models::{Candidate, Office, Election};
use std::io::{self};
use uuid::Uuid;
use std::error::Error;
use crate::save_ballot::save_to_csv;

fn create_election_ballot() -> Result<(), Box<dyn Error>> {
    println!("Creating a new election ballot...");

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
      name:election_name.clone(),
      is_open,
    };
    // saving election
    let election_headers = ["Election_ID", "Election_Name", "Is_Open"];
    
    if let Err(err) = save_to_csv(&[election], &election_headers, "elections.csv") {
        eprintln!("Failed to save elections to CSV: {}", err);
        // Handle the error appropriately, maybe return it
        return Err(err);
    }

    // Intialize offices
    let mut offices = Vec::new();
  
    loop {

        // creating offices
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
                name: candidate_name,
                office_id,
                political_party,
            };
            candidates.push(candidate);
            
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
                // Saving Candidates
                let candidates_headers = ["Candidate_ID", "Candidate_Name" ,"Office_ID", "Political_Party"];
                if let Err(err) = save_to_csv(&candidates, &candidates_headers, "candidates.csv") {
                    eprintln!("Failed to save candidates to CSV: {}", err);
                    // Handle the error appropriately, maybe return it
                    return Err(err);
                }
                break;
            }
        }

        let office = Office {
            office_id,
            office_name,
            election_id,
            election_name: election_name.clone(),
            
            //candidates,
        };
        offices.push(office);
    }
    // Saving Offices 
    let offices_headers = ["Office_ID", "Office_Name" ,"Election_ID", "Election_Name"];
    if let Err(err) = save_to_csv(&offices, &offices_headers, "offices.csv") {
        eprintln!("Failed to save offices to CSV: {}", err);
        // Handle the error appropriately, maybe return it
        return Err(err);
    }
    
    Ok(())
  
}

// Public function exposed by this module
pub fn public_interface_to_create_ballot() -> Result<(), Box<dyn Error>> {
    match create_election_ballot() {
        Ok(_) => {
            println!("Election ballot creation successful.");
            Ok(())
        },
        Err(e) => {
            eprintln!("Error occurred while creating election ballot: {}", e);
            Err(e)
        },
    }
}
