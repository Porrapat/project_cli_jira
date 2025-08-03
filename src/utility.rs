use crate::db::{JSONFileDatabase, Database};
use crate::models::{DBState, Epic, Story, Status};
use std::collections::HashMap;
use anyhow::Result;

pub fn seeding_db(filepath: &str) -> Result<()> {

    let db = JSONFileDatabase { file_path: filepath.to_owned() };

    let mut last_item_id = 1;

    let epic1 = Epic {
        name: "Example Epic".to_string(),
        description: "Testing epic".to_string(),
        status: Status::Open,
        stories: vec![2, 3],
    };

    let mut epics = HashMap::new();
    epics.insert(last_item_id, epic1);

    last_item_id = last_item_id + 1;

    let story1 = Story {
        name: "Example Story".to_string(),
        description: "Just testing".to_string(),
        status: Status::Open,
    };

    let story2 = Story {
        name: "Example Story 2".to_string(),
        description: "Just testing 2".to_string(),
        status: Status::Open,
    };



    let mut stories = HashMap::new();
    stories.insert(last_item_id, story1);
    last_item_id = last_item_id + 1;
    stories.insert(last_item_id, story2);
    last_item_id = last_item_id + 1;


    let state = DBState {
        last_item_id,
        epics,
        stories,
    };

    db.write_db(&state)?;
    println!("📦 Successfully wrote to DB");

    Ok(())
}

pub fn connecting_db(filepath: &str) {
    let db = JSONFileDatabase { file_path: filepath.to_owned() };
    match db.read_db() {
        Ok(data) => println!("✅ Loaded DB: {:?}", data),
        Err(err) => println!("❌ Failed to read DB: {}", err),
    }
}
