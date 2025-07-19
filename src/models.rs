use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {    
    Open,
    InProgress,
    Resolved,
    Closed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    // pub stories: Vec<Story>
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name,
            description,
            status: Status::Open
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            name,
            description,
            status: Status::Open
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    pub last_item_id: u32,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>
}