extern crate csv;

use std::error::Error;
use std::fs::File;
use std::process;

struct Candidate {
    name: String,
    political_party: String,
}

struct Voter {
    name: String,
    date_of_birth: String,
}

struct Office {
    office_name: String,
    candidates: Vec<Candidate>,
}

struct ElectionBallot {
    ballot_id: u32,
    election_name: String,
    offices: Vec<Office>,
    registered_voters: Vec<Voter>,
}

impl ElectionBallot {
    fn new(ballot_id: u32, election_name: String) -> ElectionBallot {
        ElectionBallot {
            ballot_id,
            election_name,
            offices: Vec::new(),
            registered_voters: Vec::new(),
        }
    }

    fn add_office(&mut self, office_name: String) {
        let office = Office {
            office_name,
            candidates: Vec::new(),
        };
        self.offices.push(office);
    }

    fn add_candidate(&mut self, office_index: usize, name: String, party: String) {
        if let Some(office) = self.offices.get_mut(office_index) {
            let candidate = Candidate {
                name,
                political_party: party,
            };
            office.candidates.push(candidate);
        } else {
            println!("Invalid office index.");
        }
    }

    fn register_voter(&mut self, name: String, date_of_birth: String) {
        let voter = Voter {
            name,
            date_of_birth,
        };
        self.registered_voters.push(voter);
    }

    fn save_to_csv(&self) -> Result<(), Box<dyn Error>> {
        // Save candidates to ballot.csv
        let mut writer = csv::Writer::from_path("ballot.csv")?;
        writer.write_record(&[
            "Ballot ID",
            "Election Name",
            "Office Name",
            "Candidate Name",
            "Political Party",
        ])?;
        for office in &self.offices {
            for candidate in &office.candidates {
                writer.write_record(&[
                    &self.ballot_id.to_string(),
                    &self.election_name,
                    &office.office_name,
                    &candidate.name,
                    &candidate.political_party,
                ])?;
            }
        }
        writer.flush()?;

        // Save voters to registered_voters.csv
        let mut voter_writer = csv::Writer::from_path("registered_voters.csv")?;
        voter_writer.write_record(&[
            "Ballot ID",
            "Election Name",
            "Voter Name",
            "Date of Birth",
        ])?;
        for voter in &self.registered_voters {
            voter_writer.write_record(&[
                &self.ballot_id.to_string(),
                &self.election_name,
                &voter.name,
                &voter.date_of_birth,
            ])?;
        }
        voter_writer.flush()?;

        Ok(())
    }
}

fn main() {
    // Read the last ballot_id from ballot.csv and increment by 1
    let last_ballot_id = match read_last_ballot_id() {
        Ok(last_id) => last_id + 1,
        Err(_) => 1, // If ballot.csv doesn't exist or cannot be read, start from 1
    };

    let mut election_ballot = ElectionBallot::new(last_ballot_id, String::from("General Election"));

    // Adding offices to the election ballot
    election_ballot.add_office(String::from("President"));
    election_ballot.add_office(String::from("Representative"));
    election_ballot.add_office(String::from("Judge"));

    // Adding candidates to each office
    election_ballot.add_candidate(0, String::from("John Doe"), String::from("Independent"));
    election_ballot.add_candidate(0, String::from("Jane Smith"), String::from("Democratic"));
    election_ballot.add_candidate(1, String::from("Alice Johnson"), String::from("Republican"));
    election_ballot.add_candidate(1, String::from("Bob Anderson"), String::from("Green Party"));
    election_ballot.add_candidate(2, String::from("Sarah Lee"), String::from("Libertarian"));

    // Registering voters
    election_ballot.register_voter(String::from("Alice"), String::from("1990-05-15"));
    election_ballot.register_voter(String::from("Bob"), String::from("1985-09-20"));
    election_ballot.register_voter(String::from("Charlie"), String::from("1978-12-10"));

    // Saving the election ballot and voter information to CSV
    if let Err(err) = election_ballot.save_to_csv() {
        eprintln!("Error writing CSV: {}", err);
        process::exit(1);
    }

    println!("Data saved to ballot.csv and registered_voters.csv");
}

fn read_last_ballot_id() -> Result<u32, Box<dyn Error>> {
    let file = File::open("ballot.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut last_id = 0;
    for result in rdr.records() {
        let record = result?;
        if let Some(id) = record.get(0) {
            last_id = id.parse::<u32>()?;
        }
    }
    Ok(last_id)
}
