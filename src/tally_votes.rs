use csv::{ReaderBuilder, Trim};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::io;

#[derive(Clone, Debug, Deserialize)]
pub struct Candidate {
    pub candidate_id: String,
    pub name: String,
    pub office_id: String,
    pub political_party: String,
    pub election_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Office {
    pub office_id: String,
    pub election_id: String,
    pub office_name: String,
    pub election_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Election {
    pub election_id: String,
    pub name: String,
    pub is_open: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Vote {
    pub vote_id: String,
    pub ballot_id: String,
    pub candidate_id: String,
    pub election_id: String,
}

pub fn tally_vote() -> Result<(), Box<dyn Error>> {
    // Load elections, candidates, and offices from CSV files
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_path("elections.csv")?;
    let elections: Vec<Election> = rdr.deserialize().filter_map(Result::ok).collect();

    let mut candidates_rdr = ReaderBuilder::new().trim(Trim::All).from_path("candidates.csv")?;
    let candidates: Vec<Candidate> = candidates_rdr.deserialize().filter_map(Result::ok).collect();

    let mut offices_rdr = ReaderBuilder::new().trim(Trim::All).from_path("offices.csv")?;
    let offices: Vec<Office> = offices_rdr.deserialize().filter_map(Result::ok).collect();

    println!("Enter the election ID:");
    let mut election_id_input = String::new();
    io::stdin().read_line(&mut election_id_input)?;
    let election_id_input = election_id_input.trim().to_string();

    // Check if election exists and is closed
    if let Some(election) = elections.iter().find(|e| e.election_id == election_id_input) {
        if election.is_open {
            println!("Election is still open. No tallying allowed at this point.");
        } else {
            // Perform tallying for each office in the election
            tally_votes_for_election(&election.election_id, &candidates, &offices);
        }
    } else {
        println!("Election not found with provided ID. Please try again.");
    }

    Ok(())
}

fn tally_votes_for_election(election_id: &String, candidates: &Vec<Candidate>, offices: &Vec<Office>) {
    let mut vote_rdr = ReaderBuilder::new().trim(Trim::All).from_path("votes.csv").expect("Unable to open votes.csv");
    let votes: Vec<Vote> = vote_rdr.deserialize().filter_map(Result::ok).collect();

    for office in offices.iter().filter(|o| o.election_id == *election_id) {
        let mut tally: HashMap<String, u32> = HashMap::new();

        for vote in votes.iter().filter(|v| v.election_id == *election_id) {
            if let Some(candidate) = candidates.iter().find(|c| c.candidate_id == vote.candidate_id && c.office_id == office.office_id) {
                *tally.entry(candidate.candidate_id.clone()).or_insert(0) += 1;
            }
        }

        if let Some((winner_id, _)) = tally.iter().max_by_key(|entry| entry.1) {
            if let Some(winner_candidate) = candidates.iter().find(|c| c.candidate_id == *winner_id) {
                println!("Office: {}, Winner: {}, Party: {}", office.office_name, winner_candidate.name, winner_candidate.political_party);
            } else {
                println!("Candidate details not found for the winner of office {}.", office.office_name);
            }
        } else {
            println!("No votes found for office {} in this election.", office.office_name);
        }
    }
}
