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
    pub user_id: u64,          // consider using a crate to generate ids
    pub name: String,
    pub date_of_birth: String, // Consider using a date type with an appropriate crate (for validation)
    pub has_voted: bool,
}
impl Voter {
    pub fn new(user_id: u64, name: String, date_of_birth: String) -> Voter {
        Voter {
            user_id,
            name,
            date_of_birth,
            has_voted: false,
        }
    }
}

pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
}
