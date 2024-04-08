extern crate csv;

use std::error::Error;
use std::process;
use std::fs::File;
use csv::Reader;

struct Candidate {
    name: String,
    political_party: String,
    votes: u32, // New field to count votes
}

struct Voter {
    user_id: String, // Unique identifier for the voter
    name: String,
    date_of_birth: String,
    has_voted: bool,
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
                votes: 0, // Initialize votes to 0
            };
            office.candidates.push(candidate);
        } else {
            println!("Invalid office index.");
        }
    }

    // update register_voter
    fn register_voter(&mut self, user_id: String, name: String, date_of_birth: String) {
        let voter = Voter {
            user_id,
            name,
            date_of_birth,
            has_voted: false,
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
            "User ID",
            "Voter Name",
            "Date of Birth",
            "Has Voted",
        ])?;
        for voter in &self.registered_voters {
            voter_writer.write_record(&[
                &self.ballot_id.to_string(),
                &self.election_name,
                &voter.user_id,
                &voter.name,
                &voter.date_of_birth,
                &voter.has_voted.to_string(),
            ])?;
        }
        voter_writer.flush()?;

        Ok(())
    }

    // Method to check if a voter is registered
  fn is_voter_registered(&self, user_id: &str) -> Result<(bool, Option<String>), Box<dyn Error>> {
          let file = File::open("registered_voters.csv")?;
          let mut rdr = Reader::from_reader(file);

          for result in rdr.records() {
              let record = result?;
              // Adjust indices according to your CSV structure
              let current_user_id = &record[2]; // Assuming column [2] is user_id
              let voter_name = &record[3]; // Assuming column [3] is voter_name
              let has_voted = record[5].parse::<bool>()?; // Assuming  column [5] is has_voted

              if current_user_id == user_id {
                  return Ok((!has_voted, Some(voter_name.to_string())));
              }
          }

          Ok((false, None)) // User ID not found or has already voted
      }
}

fn main() {
    let mut election_ballot = ElectionBallot::new(1, String::from("General Election"));

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
  // Example of registering voters with a user ID
  election_ballot.register_voter(String::from("user001"), String::from("Alice"), String::from("1990-05-15"));
  election_ballot.register_voter(String::from("user002"), String::from("Bob"), String::from("1985-09-20"));
  election_ballot.register_voter(String::from("user003"), String::from("Charlie"), String::from("1978-12-10"));


    // Saving the election ballot and voter information to CSV
    if let Err(err) = election_ballot.save_to_csv() {
        eprintln!("Error writing CSV: {}", err);
        process::exit(1);
    }

    println!("Data saved to ballot.csv and registered_voters.csv");

  // Simulation of user voting process
  let voter_id = String::from("user003"); // Assume "user005" is the current user

  match election_ballot.is_voter_registered(&voter_id) {
      Ok((is_registered, name_option)) => {
          if is_registered {
              if let Some(voter_name) = name_option {
                  println!("Welcome, {}. Please review the ballot:", voter_name);

                  // Todo:
                  // create the cast vote function inside the ElectionBallot Impl
                  // Call casting the vote method here as: 
                    // election_ballot.cast_vote_by_user_id(&voter_id);
              }
          } else {
              println!("Voter is not registered or has already voted.");
          }
      },
      Err(e) => println!("An error occurred while checking voter registration: {}", e),
  }
}
