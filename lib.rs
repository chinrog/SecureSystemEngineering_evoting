//This new position type give the option for one of 3 options.
#[derive(debug,Copy)] //There may be a need to extract the value of the position later
enum Position {
    President,
    VicePresident,
    Secretary,
}

//This election type gives us the option to possibly create a Democratic Primary, a Republican Primary, and a General Election type.
#[derive(Debug,Copy)] //There may be a need to extract the value of the election type later
enum Election_Type {
    DemocraticPrimary,
    RepublicanPrimary,
    General,
}
#[derive(Debug, Copy)] //There may be a need to copy the value of a political party later.
enum Political_Party {
    Democrat,
    Republican,
}

//This user type mod contains the types of "users" that we will be dealing with.
mod User_Types {
//this is an external crate that allows us to take in objects similar to date.  format: YYYY-MM-DD
    use chrono::NativeDate;
//Take in this object type from the outer scope.
    use super::{Position,Political_Party};

//Create a voter structure taking in the full name (need to sanitize input to deal with escape characters), date of birth (per the project
//specification, phone (to allow for OTP), and has_voted (need to query this value prior to accepting vote, should be initialized to false).
    pub struct voter {
        pub voter_full_name: String,
        pub date_Of_Birth: NativeDate,
        pub phone: String,
        pub has_voted: Bool,
    }

//This gives information about the candidate.  name position, political party, and a candidate number that will be used to identify the candidate.  When Tallying votes, this will be
//checked among all voters and used to tally votes.
    pub struct candidate {
        pub candidate_name: String,
        pub office: Position,
        pub political_party: Political_Party,
        pub candidateNum: i8,
    }
}

//Upon starting an election, wanted to give the option for multiple concurrent elections as well as different types (Democratic/Republican Primary as well as the General).
//the Vec<Candidate. option is to allow for multiple types to be added in the main. The electionNum is a way to reference the specific election.  Enums are used to control 
//input.  Further sanitization will be used upon user input.
pub mod ElectMod {
    use super::Election_Type;
    use super::User_Types;
    
    pub struct Election {
        pub election_Type: Election_Type,
        pub president: Vec<Candidate>,
        pub vicepresident: Vec<Candidate>,
        pub secretary: Vec<Candidate>,
        pub electionNum: i8,
    }
}
