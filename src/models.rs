#[derive(Debug)]
pub enum Status {
    // TODO: add fields (make sure the fields are public)
    InProgress,
    Closed
}

#[derive(Debug)]
pub struct Epic {
    // TODO: add fields (make sure the fields are public)
    pub id: u32,
    pub name: String,
    pub description: String,
    pub status: Status
}
/*

]"name": "Epic - Project 1",
  "description": "This is Project 1 for the Bootcamp",
  "status": "InProgress",
   */

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        todo!() // by default the status should be set to open and the stories should be an empty vector
    }
}

pub struct Story {
    // TODO: add fields (make sure the fields are public)
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        todo!() // by default the status should be set to open
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
}