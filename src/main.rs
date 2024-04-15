extern crate csv;

mod authenticate_admin;
mod create_ballot;
mod models;
mod save_ballot;
mod signup_user;
mod voter_registration;

use crate::authenticate_admin::authenticate;
use crate::create_ballot::public_interface_to_create_ballot;
use crate::signup_user::signup_user;
use std::io;
use std::process;
use voter_registration::interactively_register_voter;

fn main() {
    let mut authenticated = false;

    loop {
        println!("Menu:");
        if !authenticated {
            println!("0. Authenticate");
        }
        println!("1. Create Election Ballot");
        println!("2. SignUp a New User");
        println!("3. Register a New Voter");
        println!("4. Exit");
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
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 0, 1, 2, 3, or 4.");
            }
        }
    }
}
