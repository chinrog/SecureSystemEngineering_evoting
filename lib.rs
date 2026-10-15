//This new position type give the option for one of 3 options.
#[derive(Debug,Copy,Clone)] //There may be a need to extract the value of the position later
#[allow(dead_code)] // Positions are used when candidates are populated.
enum Position {
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
enum PoliticalParty {
    Democrat,
    Republican,
}

//This user type mod contains the types of "users" that we will be dealing with.
mod user_types {
//this is an external crate that allows us to take in objects similar to date.  format: YYYY-MM-DD
    use chrono::NaiveDate;
//Take in this object type from the outer scope.
    use super::{Position,PoliticalParty};

//Create a voter structure taking in the full name (need to sanitize input to deal with escape characters), date of birth (per the project
//specification, phone (to allow for OTP), and has_voted (need to query this value prior to accepting vote, should be initialized to false).
    pub struct Voter {
        pub voter_full_name: String,
        pub date_of_birth: NaiveDate,
        pub phone: String,
        pub has_voted: bool,
    }

//This gives information about the candidate.  name position, political party, and a candidate number that will be used to identify the candidate.  When Tallying votes, this will be
//checked among all voters and used to tally votes.
    pub struct Candidate {
        pub candidate_name: String,
        pub office: Position,
        pub political_party: PoliticalParty,
        pub candidate_Num: u8,
    }

    pub fn create_candidate(name: String, office: Position, party: PoliticalParty, candidate_num: u8) -> Candidate {
        Candidate {
            candidate_name: name,
            office,
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
                println!("Candidates for President: {:?}", election.president);
                println!("Candidates for Vice President: {:?}", election.vicepresident);
                println!("Candidates for Secretary: {:?}", election.secretary);
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
