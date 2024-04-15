use std::io;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use uuid::Uuid;
use csv::{Writer, ReaderBuilder, WriterBuilder};
use std::error::Error;
use regex::Regex;
use std::fs::{self, OpenOptions};

#[derive(Debug, Deserialize)]
pub struct Election {
    pub election_id: String,
    pub name: String,
    pub is_open: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Voter {
    pub user_id: String,
    pub national_id: String,
    pub name: String,
    pub date_of_birth: String,
    pub has_voted: bool,
}


fn display_open_elections() -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("elections.csv")?;

    let mut elections = Vec::new();
    for result in rdr.deserialize() {
        let election: Election = result?;
        if election.is_open {
            elections.push(election);
        }
    }

    if elections.is_empty() {
        println!("No open elections found. You can't vote at this time.");
        return Err("No open elections available.".into());
    }

    for (index, election) in elections.iter().enumerate() {
        println!("{}: {}: {}", index + 1, election.election_id, election.name);
    }

    println!("Please enter the number of the election you wish to vote in:");
    let mut selection = String::new();
    io::stdin().read_line(&mut selection)?;
    let selection: usize = selection.trim().parse().map_err(|_| "Invalid input. Please enter a valid number.")?;

    if selection == 0 || selection > elections.len() {
        return Err("Selection out of bounds. Please choose a valid election number.".into());
    }

    let selected_election = &elections[selection - 1];
    println!("You have selected: {}: {}", selected_election.election_id, selected_election.name);

    Ok((selected_election.election_id.clone(), selected_election.name.clone()))
}


#[derive(Debug, Deserialize)]
struct Candidate {
    election_id: String,
    name: String,
    office_id: String,
    political_party: String,
    candidate_id: String,
}

#[derive(Debug, Deserialize)]
struct Office {
    office_id: String,
    office_name: String,
}
fn display_candidates(election_id: String, _election_name: String, user_id: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_path("candidates.csv")?;
    let mut candidates: Vec<Candidate> = Vec::new();

    for result in rdr.deserialize() {
        let candidate: Candidate = result?;
        if candidate.election_id == election_id {
            candidates.push(candidate);
        }
    }

    let mut office_ids: Vec<String> = candidates.iter().map(|c| c.office_id.clone()).collect();
    office_ids.sort();
    office_ids.dedup();

    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_path("offices.csv")?;
    let mut offices: HashMap<String, String> = HashMap::new();

    for result in rdr.deserialize() {
        let office: Office = result?;
        if office_ids.contains(&office.office_id) {
            offices.insert(office.office_id.clone(), office.office_name);
        }
    }

    // Group candidates by their office
    let mut candidates_by_office: HashMap<String, Vec<Candidate>> = HashMap::new();
    for candidate in candidates {
        candidates_by_office.entry(candidate.office_id.clone())
            .or_insert_with(Vec::new)
            .push(candidate);
    }

    // Iterate through each office and prompt the user to make a selection
  for (office_id, office_candidates) in candidates_by_office {
  let unknown_office = "Unknown Office".to_string();
  let office_name = offices.get(&office_id).unwrap_or(&unknown_office);

  println!("For the office of '{}', please choose a candidate by entering the number next to their name:", office_name);

  for (index, candidate) in office_candidates.iter().enumerate() {
      println!("{}: {}, {}", index + 1, candidate.name, candidate.political_party);
  }

        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
        let selection: usize = selection.trim().parse().map_err(|_| "Invalid input. Please enter a valid number.")?;

        if selection == 0 || selection > office_candidates.len() {
            return Err("Selection out of bounds. Please choose a valid candidate number.".into());
        }

        let selected_candidate = &office_candidates[selection - 1];

        // Here you would call your function to record the vote
        cast_vote(election_id.clone(), user_id.clone(), selected_candidate.candidate_id.clone())?;
    }

    Ok(())
}

#[derive(Serialize)]
struct Ballot {
    ballot_id: String,
    election_id: String,
    user_id: String,
}

#[derive(Serialize)]
struct Vote {
    vote_id: String,
    ballot_id: String,
    candidate_id: String,
    election_id: String
}
fn cast_vote(election_id: String, user_id: String, candidate_id: String) -> Result<(), Box<dyn Error>> {
    let ballot_id = Uuid::new_v4().to_string();
    let vote_id = Uuid::new_v4().to_string();

    // Function to check if the file exists and is empty
    fn file_needs_headers(file_path: &str) -> bool {
        match fs::metadata(file_path) {
            Ok(metadata) => metadata.len() == 0, // File exists but is empty
            Err(_) => true, // File does not exist
        }
    }

    let ballot_file_path = "casted_ballots.csv";
    let vote_file_path = "votes.csv";

    // Check if "casted_ballots.csv" needs headers
    let mut ballot_writer = WriterBuilder::new()
        .has_headers(file_needs_headers(ballot_file_path))
        .from_writer(OpenOptions::new().append(true).create(true).open(ballot_file_path)?);
    let ballot = Ballot {
        ballot_id: ballot_id.clone(),
        election_id:election_id.clone(),
        user_id: user_id.clone(), // Clone not needed now, but keeping for consistency
    };
    ballot_writer.serialize(ballot)?;
    ballot_writer.flush()?; // Ensure the writer is flushed and resources are released

    // Check if "votes.csv" needs headers
    let mut vote_writer = WriterBuilder::new()
        .has_headers(file_needs_headers(vote_file_path))
        .from_writer(OpenOptions::new().append(true).create(true).open(vote_file_path)?);
    let vote = Vote {
        vote_id,
        ballot_id,
        candidate_id,
        election_id
    };
    vote_writer.serialize(vote)?;
    vote_writer.flush()?; // Ensure the writer is flushed and resources are released

    println!("Your vote has been successfully cast.");

    change_voter_status(user_id)?;

    Ok(())
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub date_of_birth: String,
    pub national_id: String,
    pub has_registered: bool,
    pub has_voted: bool,
}

fn change_voter_status(user_id: String) -> Result<(), Box<dyn Error>> {
    update_voter_status("registered_voters.csv", &user_id)?;
    update_user_status("users.csv", &user_id)?;
    Ok(())
}

fn update_voter_status(file_path: &str, user_id: &String) -> Result<(), Box<dyn Error>> {
    let tmp_file_path = format!("{}.tmp", file_path);
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut wtr = Writer::from_path(&tmp_file_path)?;

    for result in rdr.deserialize() {
        let mut record: Voter = result?;
        if record.user_id == *user_id {
            record.has_voted = true;
        }
        wtr.serialize(record)?;
    }

    std::fs::rename(tmp_file_path, file_path)?;
    Ok(())
}

fn update_user_status(file_path: &str, user_id: &String) -> Result<(), Box<dyn Error>> {
    let tmp_file_path = format!("{}.tmp", file_path);
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut wtr = Writer::from_path(&tmp_file_path)?;

    for result in rdr.deserialize() {
        let mut record: User = result?;
        if record.user_id.to_string() == *user_id {
            record.has_voted = true;
        }
        wtr.serialize(record)?;
    }

    std::fs::rename(tmp_file_path, file_path)?;
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

pub fn authenticate_voter() -> Result<(), Box<dyn StdError>> {
    let mut national_id = String::new();
    let mut date_of_birth = String::new();

  
  // Loop for ID number input
    loop {
        println!("Enter national ID (xxx-xxx-xxx):");
        national_id.clear(); // Clear previous input
        io::stdin().read_line(&mut national_id)?;
        national_id = national_id.trim().to_string();
  
        if validate_national_id(&national_id).is_ok() {
            break; // Exit loop if valid
        } else {
            println!("Invalid ID number format. Expected format: xxx-xxx-xxx, where x is a number from 0-9.");
        }
    }
  // Loop for date of birth input
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
  
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("registered_voters.csv")?;

    let mut user_found = false; // Track whether a user has been found

    for result in rdr.deserialize() {
        let record: Voter = result?;
        if record.national_id == national_id && record.date_of_birth == date_of_birth {
            user_found = true; // Mark that a user has been found

            if record.has_voted {
                println!("Our records indicate you have already voted. Thank you for participating!");
                return Ok(()); // Exit early since the voter has already voted
            }

            println!("Welcome! Here is a list of open elections:");
            match display_open_elections() {
                Ok((election_id, election_name)) => {
                    println!("Fetching candidates for the selected election...");
                    display_candidates(election_id, election_name, record.user_id)?;
                },
                Err(e) => {
                    return Err(e.into()); // Propagate errors from display_open_elections
                }
            }
            break; // Exit the loop since we found the user and they haven't voted yet
        }
    }

    if !user_found {
        // If no user has been found after checking all records
        return Err("User not found or unregistered. Please check your national ID and date of birth.".into());
    }

    Ok(())
}
