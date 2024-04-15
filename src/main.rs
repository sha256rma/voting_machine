extern crate csv;
extern crate env_logger;

mod authenticate_admin;
mod close_election;
mod create_ballot;
mod models;
mod open_election;
mod save_ballot;
mod signup_user;
mod voter_registration;
mod cast_vote;
mod tally_votes;

use crate::authenticate_admin::authenticate;
use crate::create_ballot::public_interface_to_create_ballot;
use crate::signup_user::signup_user;
use crate::cast_vote::authenticate_voter;
use crate::tally_votes::tally_vote;
use close_election::close_election;
use open_election::open_election;
use voter_registration::interactively_register_voter;

use log4rs;
use std::io;
use std::process;

fn main() {
    let mut authenticated = false;

    if let Some(_) = std::env::args().nth(1) {
        if let Err(e) = log4rs::init_file("../log4rs.yml", Default::default()) {
            eprintln!("{}", e);
        }
    }

    loop {
        println!("Menu:");
        if !authenticated {
            println!("0. Authenticate");
        } else {
            println!("1. Create Election Ballot");
            println!("2. Sign Up a New User");
            println!("3. Register a New Voter");
            println!("4. Open Election for Voting");
            println!("5. Close Election to Prevent Further Votes");
            println!("6. Cast Vote");
            println!("7. Tally Votes");
        }
        println!("8. Exit");
        println!("Enter your choice:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "0" => match authenticate() {
                Ok(access_granted) => {
                    if access_granted {
                        authenticated = true;
                        println!("Authentication successful!");
                    } else {
                        println!("Authentication failed!");
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            },
            "1" => {
                if authenticated {
                    if let Err(err) = public_interface_to_create_ballot() {
                        eprintln!("Error writing CSV: {}", err);
                        process::exit(1);
                    }
                } else {
                    println!("Authenticated access required!");
                }
            }
            "2" => {
                if authenticated {
                    if let Err(err) = signup_user() {
                        eprintln!("Error signing up a user: {}", err);
                        //process::exit(1);
                    }
                } else {
                    println!("Authenticated access required!");
                }
            }
            "3" => {
                if authenticated {
                    if let Err(err) = interactively_register_voter() {
                        eprintln!("Error registering voter: {}", err);
                        //process::exit(1);
                    }
                } else {
                    println!("Authenticated access required!");
                }
            }
            "4" => {
                if authenticated {
                    println!("Enter the ID of the election to open:");
                    let mut election_id = String::new();
                    io::stdin()
                        .read_line(&mut election_id)
                        .expect("Failed to read line");
                    if let Err(err) = open_election(election_id.trim()) {
                        eprintln!("Error opening election: {}", err);
                    }
                } else {
                    println!("Authenticated access required!");
                }
            }
            "5" => {
                if authenticated {
                    println!("Enter the ID of the election to close:");
                    let mut election_id = String::new();
                    io::stdin()
                        .read_line(&mut election_id)
                        .expect("Failed to read line");
                    if let Err(err) = close_election(election_id.trim()) {
                        eprintln!("Error closing election: {}", err);
                    }
                } else {
                    println!("Authenticated access required!");
                }
            }
            "6" => {
                if authenticated {
                    if let Err(err) = authenticate_voter() {
                        eprintln!("Error Casting Vote: {}", err);
                    }
                } else {
                    println!("Authenticated access required!");
                }
            }
            "7" => {
                if authenticated {
                    if let Err(err) = tally_vote() {
                        eprintln!("Error Tallying Votes: {}", err);
                    }
                } else {
                    println!("Authenticated access required!");
                }
            }
            "8" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 0, 1, 2, 3, 4, 5, 6, 7, or 8.");
            }
        }
    }
}
