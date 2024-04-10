extern crate csv;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self};
use std::process;

struct Candidate {
    name: String,
    political_party: String,
}

struct Office {
    office_name: String,
    candidates: Vec<Candidate>,
}

struct ElectionBallot {
    election_name: String,
    offices: Vec<Office>,
}

impl ElectionBallot {
    // fn new(election_name: String) -> ElectionBallot {
    //     ElectionBallot {
    //         election_name,
    //         offices: Vec::new(),
    //     }
    // }

    // fn add_office(&mut self, office_name: String, candidates: Vec<Candidate>) {
    //     let office = Office {
    //         office_name,
    //         candidates,
    //     };
    //     self.offices.push(office);
    // }

    fn save_to_csv(&self) -> Result<(), Box<dyn Error>> {
        let file_path = "ballot.csv";

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)?;

        let mut file_is_empty = false;
        let metadata = file.metadata()?;
        if metadata.len() == 0 {
            file_is_empty = true;
        }

        let mut writer = csv::WriterBuilder::new()
            .has_headers(file_is_empty)
            .from_writer(file);

        for office in &self.offices {
            for candidate in &office.candidates {
                writer.serialize((
                    &self.election_name,
                    &office.office_name,
                    &candidate.name,
                    &candidate.political_party,
                ))?;
            }
        }
        writer.flush()?;
        Ok(())
    }
}

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

fn main() {
    loop {
        println!("Menu:");
        println!("1. Create Election Ballot");
        println!("2. Exit");
        println!("Enter your choice:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => {
                let election_ballot = create_election_ballot();
                if let Err(err) = election_ballot.save_to_csv() {
                    eprintln!("Error writing CSV: {}", err);
                    process::exit(1);
                }
                println!("Election ballot data appended to ballot.csv");
            }
            "2" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1 or 2.");
            }
        }
    }
}
