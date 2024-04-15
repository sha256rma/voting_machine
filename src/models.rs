// models.rs

use crate::save_ballot::ToCsvRecord;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Election {
    pub election_id: Uuid,
    pub name: String,
    pub is_open: bool,
}

impl ToCsvRecord for Election {
    // Convert the Election instance into a CSV record
    fn to_csv_record(&self) -> Vec<String> {
        vec![
            self.election_id.to_string(),
            self.name.clone(),
            self.is_open.to_string(),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Candidate {
    pub candidate_id: Uuid,
    pub name: String,
    pub office_id: Uuid,
    pub political_party: String,
    pub election_id: Uuid,
}
impl ToCsvRecord for Candidate {
    fn to_csv_record(&self) -> Vec<String> {
        vec![
            self.candidate_id.to_string(),
            self.name.clone(),
            self.office_id.to_string(),
            self.political_party.clone(),
            self.election_id.to_string()
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Office {
    pub office_id: Uuid,
    pub election_id: Uuid, // Foreign Key linking to Election
    pub office_name: String,
    pub election_name: String,
}

impl ToCsvRecord for Office {
    fn to_csv_record(&self) -> Vec<String> {
        vec![
            self.office_id.to_string(),
            self.office_name.clone(),
            self.election_id.to_string(),
            self.election_name.clone(),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Voter {
    pub user_id: String,
    pub national_id: String,
    pub name: String,
    pub date_of_birth: String, // Consider using a date type with an appropriate crate
    pub has_voted: bool,
}
impl Voter {
    pub fn new(user_id: String, national_id: String, name: String, date_of_birth: String) -> Voter {
        Voter {
            user_id,
            national_id,
            name,
            date_of_birth,
            has_voted: false,
        }
    }
}

pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub date_of_birth: String,
    pub national_id: String,
    pub has_registered: bool,
    pub has_voted: bool,
}
impl User {
    pub fn new(name: String, date_of_birth: String, national_id: String) -> User {
        User {
            user_id: Uuid::new_v4(),
            name,
            date_of_birth,
            national_id,
            has_registered: false,
            has_voted: false,
        }
    }
}
