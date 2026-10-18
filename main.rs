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
• View an audit log of all election lifecycle events and voter registrations, including timestamps. 
============================================================================================*/

//use std::io;
mod lib;
use lib::system::sanitize_u8;
//use lib::elect_mod::{create_election, list_existing_elections, Election};
//use lib::user_types::{create_candidate, create_voter, Candidate, Voter};

fn mainmenu() -> u8 {
    println!("========================================");
    println!("       E-Voting App (admin portal)");
    println!("========================================");
    println!("Please select an option:");
    println!("1. List Existing Elections");
    println!("2. Create Election");
    println!("3. Create Voter");
    println!("4. View system logs");
    println!("5. view election logs");
    println!("6. view user logs");
    println!("7. Exit Program");
    println!("========================================");
    println!("Enter your choice (1-7):");
    println!("========================================");
    //let menu_string = String::from();

    sanitize_u8(7)

}

fn main() {
//this function takes place post authentication
//call menu function
    let mut choice = mainmenu();

    if choice == 0 {
        // If the input was invalid, prompt the user again
        choice = sanitize_u8(7);
    }

    println!("You selected option: {}", choice);

//Option one:  check existing elections
    match choice {
        1 => {
            println!("Listing existing elections...");
            // Call a function to list existing elections
            //list_existing_elections();
        }
    //Search DB for elections that were already created.
    //If no elections, print "no elections, please choose option 2"
    //If elections exist, print the type, the candidates, and whether the election is open or not.
        2 => {
            println!("Creating a new election...");
            // Call a function to create a new election
            //create_election();
        }
        3 => {
            println!("Create a new voter...");
            // Call a function to create a new voter
            //create_voter();
        }
        4 => {
            println!("Fetching System logs...");
            // Call a function to create a new voter
            //logmenu();
        }
        5 => {
            println!("Fetching Election logs...");
            // Call a function to create a new voter
            //logmenu();
        }
        6 => {
            println!("Fetching User logs menu...");
            // Call a function to create a new voter
            //logmenu();
        }
        7 => {
            println!("Exiting the program...");
            // Exit the program
            std::process::exit(0);
        }
        _ => unreachable!(), // This case should never happen due to earlier validation
    }

}
