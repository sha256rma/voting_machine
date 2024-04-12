extern crate csv;

mod models;
mod save_ballot;
mod create_ballot;
mod voter_registration;
mod signup_user;

use crate::create_ballot::public_interface_to_create_ballot;
use crate::save_ballot::save_election_ballot_to_csv;
use crate::signup_user::signup_user;
use voter_registration::interactively_register_voter;
use std::process;
use std::io;

fn main() {
    loop {
        println!("Menu:");
        println!("1. Create Election Ballot");
        println!("2. SignUp a New User");
        println!("3. Register a New Voter");
        println!("4. Exit");
        println!("Enter your choice:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                let election_ballot = public_interface_to_create_ballot();
                if let Err(err) = save_election_ballot_to_csv(&election_ballot) {
                    eprintln!("Error writing CSV: {}", err);
                    process::exit(1);
                }
                println!("Election ballot data appended to ballot.csv");
            },
            "2" => {
                if let Err(err) = signup_user() {
                    eprintln!("Error signing up a user: {}", err);
                    process::exit(1);
                }
            },
            "3" => {
                if let Err(err) = interactively_register_voter() {
                    eprintln!("Error registering voter: {}", err);
                    process::exit(1);
                }
            },
            "4" => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Invalid choice. Please enter 1, 2, 3, or 4.");
            }
        }
    }
}
