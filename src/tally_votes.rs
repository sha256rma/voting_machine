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

    loop {
        println!("Enter the election ID:");
        let mut election_id_input = String::new();
        io::stdin().read_line(&mut election_id_input)?;
        let election_id_input = election_id_input.trim().to_string();

        println!("Enter the election name:");
        let mut election_name_input = String::new();
        io::stdin().read_line(&mut election_name_input)?;
        let election_name_input = election_name_input.trim().to_string();

        // Check if election exists and is closed
        if let Some(election) = elections.iter().find(|e| e.election_id == election_id_input && e.name == election_name_input) {
            if election.is_open {
                println!("Election is still open. No tallying allowed at this point.");
                break;
            } else {
                // Perform tallying
                tally_votes_for_election(&election.election_id, &candidates, &offices);
                break;
            }
        } else {
            println!("Election not found with provided ID and name. Type 'quit' to exit or press Enter to retry.");
            let mut decision = String::new();
            io::stdin().read_line(&mut decision)?;
            if decision.trim().eq_ignore_ascii_case("quit") {
                break;
            }
        }
    }

    Ok(())
}

fn tally_votes_for_election(election_id: &String, candidates: &Vec<Candidate>, offices: &Vec<Office>) {
    let mut vote_rdr = ReaderBuilder::new().trim(Trim::All).from_path("votes.csv").expect("Unable to open votes.csv");
    let votes: Vec<Vote> = vote_rdr.deserialize().filter_map(Result::ok).collect();

    let mut tally: HashMap<String, u32> = HashMap::new();
    for vote in votes.iter().filter(|v| v.election_id == *election_id) {
        *tally.entry(vote.candidate_id.clone()).or_insert(0) += 1;
    }

    let winner = tally.iter().max_by_key(|entry| entry.1);
    if let Some((winner_id, _)) = winner {
        if let Some(winner_candidate) = candidates.iter().find(|c| c.candidate_id == *winner_id) {
            if let Some(winner_office) = offices.iter().find(|o| o.office_id == winner_candidate.office_id) {
                println!("Winner: {}, Office: {}, Party: {}", winner_candidate.name, winner_office.office_name, winner_candidate.political_party);
            } else {
                println!("Office details not found for the winning candidate.");
            }
        } else {
            println!("Candidate details not found for the winner.");
        }
    } else {
        println!("No votes found for this election.");
    }
}
