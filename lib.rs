use std::io;

//This new position type give the option for one of 3 options.
#[derive(Debug,Copy,Clone)] //There may be a need to extract the value of the position later
#[allow(dead_code)] // Positions are used when candidates are populated.
pub enum PoliticalOffice {
    President,
    VicePresident,
    Secretary,
}

//This election type gives us the option to possibly create a Democratic Primary, a Republican Primary, and a General Election type.
#[derive(Debug,Copy,Clone)] //There may be a need to extract the value of the election type later
pub enum ElectionType {
    DemocraticPrimary,
    RepublicanPrimary,
    General,
}
#[derive(Debug, Copy, Clone)] //There may be a need to copy the value of a political party later.
#[allow(dead_code)] //Variants are used when candidates are populated.
pub enum PoliticalParty {
    Democrat,
    Republican,
}

//This user type mod contains the types of "users" that we will be dealing with.
mod user_types {
//this is an external crate that allows us to take in objects similar to date.  format: YYYY-MM-DD
    use chrono::NaiveDate;
//Take in this object type from the outer scope.
    use super::{PoliticalOffice,PoliticalParty};

//Create a voter structure taking in the full name (need to sanitize input to deal with escape characters), date of birth (per the project
//specification, phone (to allow for OTP), and has_voted (need to query this value prior to accepting vote, should be initialized to false).
    #[allow(dead_code)]    
    pub struct Voter {
        pub voter_full_name: String,
        pub date_of_birth: NaiveDate,
        pub phone: String,
        pub has_voted: bool,
    }

//This gives information about the candidate.  name position, political party, and a candidate number that will be used to identify the candidate.  When Tallying votes, this will be
//checked among all voters and used to tally votes.
    #[allow(dead_code)]
    pub struct Candidate {
        pub candidate_name: String,
        pub office: PoliticalOffice,
        pub political_party: PoliticalParty,
        pub candidate_Num: u8,
    }

    pub fn create_candidate(name: String, office: PoliticalOffice, party: PoliticalParty, candidate_num: u8) -> Candidate {
        
        println!("Enter the name of the candidate...");
        
        Candidate {
            candidate_name: name,
            office: office,
            political_party: party,
            candidate_Num: candidate_num,
        }
    }

    pub fn create_voter(name: String, dob: NaiveDate, phone: String) -> Voter {
        Voter {
            voter_full_name: name,
            date_of_birth: dob,
            phone,
            has_voted: false,
        }
    }
}

//Upon starting an election, wanted to give the option for multiple concurrent elections as well as different types (Democratic/Republican Primary as well as the General).
//the Vec<Candidate. option is to allow for multiple types to be added in the main. The electionNum is a way to reference the specific election.  Enums are used to control 
//input.  Further sanitization will be used upon user input.
pub mod elect_mod {
    use super::ElectionType;
    use super::user_types::Candidate;
    
    pub struct Election {
        pub election_type: ElectionType,
        pub president: Vec<Candidate>,
        pub vicepresident: Vec<Candidate>,
        pub secretary: Vec<Candidate>,
        pub election_num: u8,
        pub elect_open: bool,
    }

    pub fn create_election(election_type: ElectionType, election_num: u8) -> Election {
        println!("Each new election should support at least 3 customizable positions for"); 
        println!("election (e.g. President, Vice President, Secretary)");
        println!("Each position should have at least 2 possible candidates for election");
        println!("Each candidate must have fields for a name and political party");
    
        println!("Creating a new election...");
        println!("Creating a new election...");



        Election {
            election_type,
            president: Vec::new(),
            vicepresident: Vec::new(),
            secretary: Vec::new(),
            election_num,
            elect_open: false,
        }
    }

    pub fn list_existing_elections(elections: &Vec<Election>) {
        if elections.is_empty() {
            println!("No elections found. Please create an election first.");
        } else {
            for election in elections {
                println!("Election Number: {}", election.election_num);
                println!("Election Type: {:?}", election.election_type);
                println!("Election Open: {}", election.elect_open);
                //println!("Candidates for President: {:?}", election.president);
                //println!("Candidates for Vice President: {:?}", election.vicepresident);
                //println!("Candidates for Secretary: {:?}", election.secretary);
                println!("----------------------------------------");
            }
        }
    }
}

pub mod program_initdb{
    //create and initialize the sqlite database.

}

pub mod program_logging{
    //create and initialize the sqlite database for logging.
    //create a function to log events to the database.
    //create a function to view logs from the database.
}

pub fn sanitize_u8() -> u8 {
/* This function is vitial.  By using escape characters or flooding the variable with more data than is expected, in other programming languages
this would cause a buffer flow.  In rust and some of the more modern languages, the application can fail.  This function deals with the E in stride which
prevents Escalation of privilege.  Using techniques such as Injection attacks, it's possible to bypass the user interface and gain console access to the
system running the app where they can run commands.  If a data type is expecting a smaller value than what the threat actor provided then the threat 
actor could perform a buffer overflow attack and write to adjacent memory.  Specifically, the line: 

let mut input = String::new();

creates an empty string that fills up with additional input as needed. The below line 151 deal with detecting control characters and escape 
characters. If the input is empty or not a number, the user is prompted to enter a number between the min and max values.  
The match statement checks if the input is a valid number within the specified range.  If it is, the function returns that number.  If not,
 an error message is displayed, and the loop continues until valid input is received.  This function ensures that only valid numeric input 
 is accepted, preventing potential security vulnerabilities related to invalid or malicious input.
 
*/
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    if input.chars().any(|c| c.is_control()) {
        eprintln!("Error: Control character/escape character detected!");
        return 0; // Return 0 to indicate an error
    }

    if input.is_empty() || !input.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Error: Only numbers from 1-7 are accepted");
        return 0; // Return 0 to indicate an error
    }

    match input.parse::<u8>() {
        Ok(num) if (1..=7).contains(&num) => return num,
        _ => eprintln!("Error: Invalid choice. Please enter a number between 1 and 7."),
    }
    let mut input_u8 = input.parse::<u8>().unwrap_or(0);

    return input_u8;
}

pub fn sanitize_name() -> String {
/* 
 
*/
    let mut input = String::new();
    let error: String = String::from("bad_input");
    
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    if input.chars().any(|c| c.is_control()) {
        println!("Error: Control character/escape character detected!");
        return error; // Return "bad_input" to indicate an error
    }

    if input.is_empty() || !input.chars().all(|c| c.is_ascii_alphabetic()) {
        eprintln!("Error: Only strings in the format <Firstname Lastname> are accepted");
        return error; // Return "bad_input" to indicate an error
    }

    match input.trim() {
        s if !s.is_empty()
            && s.split_whitespace().count() == 2
            && s.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) =>
        {
            return s.to_string();
        }
    _ => eprintln!("Error: Invalid name. Please enter a first and last name using letters only."),
    }

    let mut input_name: String = input.parse::<String>().unwrap_or(" ");

    return input_name;
}
