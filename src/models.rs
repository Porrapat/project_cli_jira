// #[derive(Debug)]
pub enum Status {
    // TODO: add fields (make sure the fields are public)
    Open,
    InProgress,
    Resolved,
    Closed
}
//  `status` can be `Open`, `InProgress`, `Resolved` or `Closed`. 
// #[derive(Debug)]
pub struct Epic {
    // TODO: add fields (make sure the fields are public)
    // pub id: u32,
    pub name: String,
    pub description: String,
    pub status: Status
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
// todo!() // by default the status should be set to open and the stories should be an empty vector

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

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    pub last_item_id: u32,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>
}