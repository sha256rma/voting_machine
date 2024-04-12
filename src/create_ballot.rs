use crate::models::{Candidate, Office, ElectionBallot};
use std::io::{self};

fn create_election_ballot() -> ElectionBallot {
    println!("Creating a new election ballot...");

    println!("Enter the election name:");
    let mut election_name = String::new();
    io::stdin()
        .read_line(&mut election_name)
        .expect("Failed to read line");
    let election_name = election_name.trim().to_string();

    let mut offices = Vec::new();

    loop {
        println!("Enter the office name (or type 'done' to finish adding offices):");
        let mut office_name = String::new();
        io::stdin()
            .read_line(&mut office_name)
            .expect("Failed to read line");
        let office_name = office_name.trim().to_string();

        if office_name == "done" {
            break;
        }

        let mut candidates = Vec::new();
        loop {
            println!("Enter the candidate name for office '{}':", office_name);
            let mut candidate_name = String::new();
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
                name: candidate_name,
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
                break;
            }
        }

        let office = Office {
            office_name,
            candidates,
        };
        offices.push(office);
    }

    ElectionBallot {
        election_name,
        offices,
    }
}

// Public function exposed by this module
pub fn public_interface_to_create_ballot() -> ElectionBallot {
    create_election_ballot() // This calls the private function
}
