// models.rs

pub struct Candidate {
    pub name: String,
    pub political_party: String,
}

pub struct Office {
    pub office_name: String,
    pub candidates: Vec<Candidate>,
}

pub struct ElectionBallot {
    pub election_name: String,
    pub offices: Vec<Office>,
}
pub struct Voter {
    pub national_id: String,          // consider using a crate to generate ids
    pub name: String,
    pub date_of_birth: String, // Consider using a date type with an appropriate crate
    pub has_voted: bool,
}
impl Voter {
    pub fn new(national_id: String, name: String, date_of_birth: String) -> Voter {
        Voter {
            national_id,
            name,
            date_of_birth,
            has_voted: false,
        }
    }
}

pub struct User {
    pub name: String,
    pub date_of_birth: String,
    pub national_id: String,
    pub password: String,
    pub has_registered: bool,
    pub has_voted: bool,
    
}
impl User {
    pub fn new(name: String, date_of_birth: String, national_id: String, password: String) -> User {
        User {
            name,
            date_of_birth,
            national_id,
            password,
            has_registered: false,
            has_voted: false,
          
        }
    }
}
