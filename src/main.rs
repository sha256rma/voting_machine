use std::io;

// Dummy functions
fn dummy_function_1() {
    println!("Casting a ballot...");
}

fn dummy_function_2() {
    println!("Tallying the votes...");
}

fn dummy_function_3() {
    println!("Closing the election...");
}

fn main() {
    loop {
        println!("Menu:");
        println!("1. Cast a ballot");
        println!("2. Tally the votes");
        println!("3. Close the election");
        println!("4. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice {
            1 => {
                dummy_function_1();
            }
            2 => {
                dummy_function_2();
            }
            3 => {
                dummy_function_3();
            }
            4 => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice! Please enter a number between 1 and 4."),
        }
    }
}
