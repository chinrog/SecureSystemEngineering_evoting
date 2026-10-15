/*=============================PROJECT RUBRIC - ADMIN PORTAL==============================
Election Admins must be able to:
• Authenticate into an admin-only interface.
• Create a new election.
    – Each new election should support at least 3 customizable positions for election (e.g., President, Vice President, 
    Secretary or Best Sandwich, Best Taco, Best Hamburger).
    – Each position should have at least 2 possible candidates for election (e.g., Subway, Jersey
    Mike’s, Firehouse Subs).
    – Each candidate must have fields for a name and political party.
• Register new voters.
    – Create a new voter using a full name and date of birth.
    – Check that this voter does not already exist.
    – Assign the voter a unique numeric PIN upon registration. This PIN is provided to the voter and is required to 
    authenticate before casting a vote.
• View an audit log of all election lifecycle events and voter registrations, including timestamps. */

use std::io;
use crate::elect_mod::{create_election, list_existing_elections, Election};
use crate::user_types::{create_candidate, create_voter, Candidate, Voter};

fn mainmenu() -> u8 {
    println!("========================================");
    println!("       E-Voting App (admin portal)");
    println!("========================================");
    println!("Please select an option:");
    println!("1. List Existing Elections");
    println!("2. Create Election");
    println!("3. Create Voter");
    println!("4. View Audit Log");
    println!("5. Exit Program");
    println!("Enter your choice (1-5):");

    let menu_string = String::from("main");

    sanitize_menu_input(menu_string)

}

fn logmenu(){
    println!("========================================");
    println!("       E-Voting App (admin portal_logging)");
    println!("========================================");
    println!("Please select an option:");
    println!("1. view election logs");
    println!("2. view user logs");
    println!("3. Back to main menu");
    println!("Enter your choice (1-3):");   
    
    let menu_string = String::from("log");
}

fn sanitize_menu_input(menu_string: String) -> u8 {
/* This function is vitial.  By using escape characters or flooding the variable with more data than is expected, in other programming languages
this would cause a buffer flow.  In rust and some of the more modern languages, the application can fail.  This function deals with the E in stride which
prevents Escalation of privilege.  Using techniques such as Injection attacks, it's possible to bypass the user interface and gain console access to the
system running the app where they can run commands.  If a data type is expecting a smaller value than what the threat actor provided then the threat 
actor could perform a buffer overflow attack and write to adjacent memory.  Specifically, the line 

let mut input = String::new();

creates an empty string that fills up with additional input as needed. The below line 73 deal with detecting control characters and escape 
characters. If the input is empty or not a number, the user is prompted to enter a number between the min and max values.  
The match statement checks if the input is a valid number within the specified range.  If it is, the function returns that number.  If not,
 an error message is displayed, and the loop continues until valid input is received.  This function ensures that only valid numeric input 
 is accepted, preventing potential security vulnerabilities related to invalid or malicious input.

then 
*/
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    if input.chars().any(|c| c.is_control()) {
        eprintln!("Error: Control character/escape character detected!");
        continue;
    }

    let (min, max) = match menu_string.as_str() {
        "main" => (1, 5),
        "log" => (1, 3),
        _ => return 0,
    };

    if input.is_empty() || !input.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Error: Only numbers from {min}-{max} are accepted");
        continue;
    }

    match input.parse::<u8>() {
        Ok(num) if (min..=max).contains(&num) => return num,
        _ => eprintln!("Error: Invalid choice. Please enter a number between {min} and {max}."),
    }
    input;
}

fn main() {
//this function takes place post authentication
//call menu function
    let choice = mainmenu();

    println!("You selected option: {}", choice);

//condition to catch logging menu
    if choice == 4 {
        let choice = logmenu();
    }
//Option one:  check existing elections
    match choice {
        1 => {
            println!("Listing existing elections...");
            // Call a function to list existing elections
            list_existing_elections();
        }
    //Search DB for elections that were already created.
    //If no elections, print "no elections, please choose option 2"
    //If elections exist, print the type, the candidates, and whether the election is open or not.
        2 => {
            println!("Creating a new election...");
            // Call a function to create a new election
            create_election();
        }
        3 => {
            println!("Create a new voter...");
            // Call a function to create a new voter
            create_voter();
        }
        4 => {
            println!("Audit menu...");
            // Call a function to create a new voter
            logmenu();
        }
        5 => {
            println!("Exiting the program...");
            // Exit the program
            std::process::exit(0);
        }
        _ => unreachable!(), // This case should never happen due to earlier validation
    }

}
