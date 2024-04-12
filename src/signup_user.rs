use crate::models::User;
use rand::rngs::OsRng;
use std::error::Error;
use std::io::{self};

use csv::{Reader, Writer};
use std::fs::OpenOptions;
use std::path::Path;

use argon2::{
    password_hash::{
        PasswordHasher, SaltString
    },
    Argon2
};


fn user_exists(username: &str, email: &str) -> Result<bool, Box<dyn Error>> {
    if !Path::new("users.csv").exists() {
        return Ok(false);
    }

    let mut rdr = Reader::from_path("users.csv")?;
    for result in rdr.records() {
        let record = result?;
        if record.get(0).map(|r| r == username).unwrap_or(false) || record.get(2).map(|r| r == email).unwrap_or(false) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn save_user(user: &User) -> Result<(), Box<dyn Error>> {
    let file_path = "users.csv";
    let file_exists = Path::new(file_path).exists();
    let file = OpenOptions::new().append(true).create(true).open("users.csv")?;

    let mut wtr = Writer::from_writer(file);
  
    // If the file did not previously exist, write the header
    if !file_exists {
        wtr.write_record(&["username", "password", "email"])?;
    }
    wtr.write_record(&[
      &user.username, 
      &user.password,
      &user.email,
    ])?;
  
    wtr.flush()?;
    println!("{} has been created successfully.", user.username);
    Ok(())
}

pub fn signup_user() -> Result<User, Box<dyn Error>> {
    let mut username = String::new();
    let mut password = String::new();
    let mut email = String::new();

    println!("Enter username:");
    io::stdin().read_line(&mut username)?;

    println!("Enter password:");
    io::stdin().read_line(&mut password)?;

    println!("Enter email:");
    io::stdin().read_line(&mut email)?;

    username = username.trim().to_string();
    email = email.trim().to_string();

    if user_exists(&username, &email)? {
        return Err("Username or email already exists.".into());
    }

    // Generate a salt using SaltString
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = match argon2.hash_password(password.trim().as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => return Err(e.to_string().into()), // Convert the error to a String and then into a Box<dyn Error>
    };

    let user = User {
        username,
        password: password_hash, // Store the hashed password
        email,
    };

    // Call save_user to save the newly created user
    save_user(&user)?;

    // Return the user instance after saving
    Ok(user)
}
