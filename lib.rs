
//This user type mod contains the types of "users" that we will be dealing with.
mod user_types {

    use super::system::sanitize_name;
//this is an external crate that allows us to take in objects similar to date.  format: YYYY-MM-DD
    use chrono::NaiveDate;
    use super::elect_mod::*;
    
//Create a voter structure taking in the full name (need to sanitize input to deal with escape characters), date of birth (per the project
//specification), phone (to allow for OTP), and has_voted (need to query this value prior to accepting vote, should be initialized to false).
    #[allow(dead_code)]    
    pub struct Voter {
        pub voter_full_name: String,
        pub date_of_birth: NaiveDate,
        pub voter_id: String,
        pub phone: String,
        pub has_voted: bool,
    }

//This gives information about the candidate.  name position, political party, and a candidate number that will be used to identify the candidate.  When Tallying votes, this will be
//checked among all voters and used to tally votes.
    #[allow(dead_code)]
    pub struct Candidate {
        //use super::elect_mod::{PoliticalOffice,PoliticalParty};

        pub candidate_name: String,
        pub office: PoliticalOffice,
        pub political_party: PoliticalParty,
        pub candidate_Num: String,
    }

    pub fn create_candidate() -> Candidate {
        use super::system::{generate_numeric_string,sanitize_name};
        use super::elect_mod::{PoliticalOffice,PoliticalParty,specify_political_office,specify_political_party};
        println!("Enter the name of the candidate...");
        let name = sanitize_name();
        println!("Enter the political office of the candidate...");
        let mut office = specify_political_office();

        println!("Enter the political party of the candidate...");
        let mut party = specify_political_party();

        println!("Enter the name of the candidate...");
        let cuid: String = generate_numeric_string();

        Candidate {
            candidate_name: name,
            office: office,
            political_party: party,
            candidate_Num: cuid,
        }
    }

    pub fn create_voter() -> Voter {
        use chrono::NaiveDate;
        use super::user_types::Voter;
        use super::system::{generate_numeric_string,sanitize_strnum,set_date};

        println!("Enter the name of the candidate...");
        let name = sanitize_name();

        println!("generating VID");
        let vuid: String = generate_numeric_string();

        println!("Enter the voter's phone number");
        let mut phone: String = sanitize_strnum();
        if phone == "bad_input"{
            phone = sanitize_strnum();
        }

        println!("As a requirement, we need all voters to specify their birthdays");
        let mut dob: NaiveDate = set_date();

        Voter {
            voter_full_name: name,
            date_of_birth: dob,
            voter_id: vuid,
            phone: phone,
            has_voted: false,
        }
    }
}

//Upon starting an election, wanted to give the option for multiple concurrent elections as well as different types (Democratic/Republican Primary as well as the General).
//the Vec<Candidate. option is to allow for multiple types to be added in the main. The electionNum is a way to reference the specific election.  Enums are used to control 
//input.  Further sanitization will be used upon user input.
pub mod elect_mod {
    use super::user_types::create_candidate;

use super::system::sanitize_u8;

use super::user_types::Candidate;

    pub enum PoliticalParty {
        Democrat,
        Republican,
        Independent,
        ppnotSpecified,
    }
    pub enum PoliticalOffice {
        President,
        VicePresident,
        Secretary,
        ponotSpecified,
    }

    #[derive(Debug,Copy,Clone)]
    pub enum ElectionType {
        DemocraticPrimary,
        RepublicanPrimary,
        General,
        etnotSpecified,
    }
    
    pub struct Election {
        pub election_type: ElectionType,
        pub president: Vec<Candidate>,
        pub vicepresident: Vec<Candidate>,
        pub secretary: Vec<Candidate>,
        pub election_num: String,
        pub elect_open: bool,
    }

    pub fn create_election() -> Election {
        use super::elect_mod::*;
        use super::system::generate_numeric_string;
        let mut president: Vec<Candidate> = Vec::new();
        let mut vicepresident: Vec<Candidate> = Vec::new();
        let mut secretary: Vec<Candidate> = Vec::new();

        println!("Each new election should support at least 3 customizable positions for the election (e.g. President,"); 
        println!("Vice President, Secretary).");      
        println!("Each position should have at least 2 possible candidates for election");
        println!("Each candidate must have fields for a name and political party");
        println!("How many tickets are running in the election (maximum number is 3).");
        let mut input = sanitize_u8(3);
        
        for i in 1..input{
            println!("Let's customize the {}, ticket.", i);
            for j  in 1..=3 {
                println!("Enter the Presidental candidate for this ticketJ:");
                let mut candidate: Candidate = create_candidate();
                president.push(candidate);

                println!("Enter the Vice Presidential candidate for this ticket:");
                let mut candidate: Candidate = create_candidate();
                vicepresident.push(candidate);
                
                println!("Enter the Secretarial candiate for this ticket:");
                let mut candidate: Candidate = create_candidate();
                secretary.push(candidate);
            }
        }

        println!("What type of election is this?");
        let mut elect_input: ElectionType = specify_election_type();

        println!("Generating election number...");
        let electnum = generate_numeric_string();
        
        println!("Is this election open?");
        

        Election {
            election_type: elect_input,
            president: president,
            vicepresident: vicepresident,
            secretary: secretary,
            election_num: electnum,
            elect_open: false,
        }
    }

    pub fn list_existing_elections(elections: &Vec<Election>) {
        //use super::elect_mod::*
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



    pub fn specify_political_office() -> PoliticalOffice {
        use super::system::sanitize_u8;
        use super::elect_mod::PoliticalOffice::*;
        println!("Please select an option:");
        println!("1. President");
        println!("2. Vice President");
        println!("3. Secretary");
        println!("========================================");
        println!("Enter your choice (1-3):");
        println!("========================================");
    
        let mut choice = sanitize_u8(3);
        
        if choice == 0 {
            // If the input was invalid, prompt the user again
            println!("An invalid entry was submitted. Enter your choice (1-3):");
            choice = sanitize_u8(3);
        }

        match choice {
            1 => return PoliticalOffice::President,
            2 => return PoliticalOffice::VicePresident,
            3 => return PoliticalOffice::Secretary,
            _ => {println!("Invalid choice. Enter 1, 2, or 3"); return ponotSpecified},
        }
    }

    pub fn specify_election_type() -> ElectionType {
        use super::system::sanitize_u8;
        use super::elect_mod::ElectionType::*;
        println!("Please select an option:");
        println!("1. Democratic Primary");
        println!("2. Republican Primary");
        println!("3. General Election");
        println!("========================================");
        println!("Enter your choice (1-3):");
        println!("========================================");
    
        let mut choice = sanitize_u8(3);
        
        if choice == 0 {
            // If the input was invalid, prompt the user again
            println!("An invalid entry was submitted. Enter your choice (1-3):");
            choice = sanitize_u8(3);
        }

        match choice {
            1 => return ElectionType::DemocraticPrimary,
            2 => return ElectionType::RepublicanPrimary,
            3 => return ElectionType::General,
            _ => {println!("Invalid choice. Enter 1, 2, or 3"); return etnotSpecified},
        }
    }

    pub fn specify_political_party() -> PoliticalParty{
        use super::elect_mod::PoliticalParty::*;
        use super::system::sanitize_u8;
    
        println!("Please select an option:");
        println!("1. Democrat");
        println!("2. Republican");
        println!("3. Independent");
        println!("========================================");
        println!("Enter your choice (1-3):");
        println!("========================================");

        let mut choice = sanitize_u8(3);

        if choice == 0 {
            // If the input was invalid, prompt the user again
            println!("An invalid entry was submitted. Enter your choice (1-2):");
            choice = sanitize_u8(2);
        }

        match choice{
            1 => return PoliticalParty::Democrat,
            2 => return PoliticalParty::Republican,
            3 => return PoliticalParty::Independent,
            _ => {println!("Invalid choice. Enter 1, or 3"); return ppnotSpecified},
        }
    }
}

pub mod system{
    use rand::Rng;
    use chrono::NaiveDate;
    use std::io;
    /*pub mod program_initdb{
        //create and initialize the sqlite database.

    }*/

    /*pub mod program_logging{
        //create and initialize the sqlite database for logging.
        //create a function to log events to the database.
        //create a function to view logs from the database.
    }*/

    /* These functions are vitial.  By using escape characters or flooding the variable with more data than is expected, in other programming languages
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
    pub fn sanitize_u8(limiter: u8) -> u8 {

        let mut input = String::new();
        
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.chars().any(|c| c.is_control()) {
            eprintln!("Error: Control character/escape character detected!");
            return 0; // Return 0 to indicate an error
        }

        if input.is_empty() || !input.chars().all(|c| c.is_ascii_digit()) {
            eprintln!("Error: Only numbers from 1-{} are accepted", limiter);
            return 0; // Return 0 to indicate an error
        }

        match input.parse::<u8>() {
            Ok(num) if (1..=limiter).contains(&num) => return num,
            _ => eprintln!("Error: Invalid choice. Please enter a number between 1 and {}.", limiter),
        }
        let mut input_u8 = input.parse::<u8>().unwrap_or(0);

        return input_u8;
    }

    pub fn sanitize_name() -> String {

        let mut input = String::new();
        let error: String = String::from("bad_input");
        
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.chars().any(|c| c.is_control()) {
            println!("Error: Control character/escape character detected!");
            return error; // Return "bad_input" to indicate an error
        }

        let parts: Vec<&str> = input.split_whitespace().collect();  // Require exactly two name parts, containing letters only.

        if parts.len() != 2 || !parts.iter().all(|part| {part.chars().all(|c| c.is_alphabetic())})
            {
                eprintln!("Error: Enter a first and last name using letters only.");
                return String::from("bad_input");
            }

        // Return the validated name.
        return input.to_string();
    }

    pub fn generate_numeric_string() -> String {
        let length:u16 = 10;
        let mut rng = rand::rng();

        (0..length)
            .map(|_| rng.random_range(0..10).to_string())
            .collect()
    }

    pub fn set_date() -> NaiveDate {
        use std::io;

        println!("Please be sure to enter the date in the format YYYY-MM-DD:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        match NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                println!("Invalid date format. Please enter the date in the format YYYY-MM-DD.");
                set_date() // loop until correct date is inserted.
            }
        }
    }

    pub fn sanitize_strnum() -> String {

        let mut input = String::new();
        let error: String = String::from("bad_input");
        
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.chars().any(|c| c.is_control()) {
            println!("Error: Control character/escape character detected!");
            return error; // Return "bad_input" to indicate an error
        }

        if input.len() != 7 || !input.chars().all(|c| c.is_ascii_digit()) {
            eprintln!("Error: Enter exactly 7 digits.");
            return String::from("bad_input");
        }

        // Return the validated name.
        return input.to_string();
    }

}
