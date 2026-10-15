//This new position type give the option for one of 3 options.
enum Position {
    President,
    VicePresident,
    Secretary,
}

//This election type gives us the option to possibly create a Democratic Primary, a Republican Primary, and a General Election type.
enum Election_Type {
    DemocraticPrimary,
    RepublicanPrimary,
    General
}

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

//
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
