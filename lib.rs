enum Position {
    President,
    VicePresident,
    Secretary,
}

enum Election_Type {
    DemocraticPrimary,
    RepublicanPrimary,
    General
}

mod User_Types {
    use chrono::NativeDate;
    use super::{Position};

    pub struct voter {
        pub voter_full_name: String,
        pub date_Of_Birth: NativeDate,
        pub phone: String,
        pub has_voted: Bool,
    }

    pub struct candidate {
        pub candidate_name: String,
        pub office: Position,
        pub political_party: String,
        pub candidateNum: i8,
    }
}

pub mod Election {
    use super::Election_Type;
    use super::User_Types;

    pub election_Type: Election_Type,
    pub president: Vec<Candidate>,
    pub vicepresident: Vec<Candidate>,
    pub secretary: Vec<Candidate>,
    pub electionNum: i8,
}