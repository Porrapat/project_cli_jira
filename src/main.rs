mod db;
mod models;

use db::{JSONFileDatabase, Database}; // อย่าลืม `pub` ใน db.rs
use crate::models::{DBState, Epic, Story, Status};
use std::collections::HashMap;
use anyhow::Result;

fn seeding_db(filepath: &str) -> Result<()> {

    let db = JSONFileDatabase { file_path: filepath.to_owned() };
    // let db = JSONFileDatabase { file_path: "db3.json".to_string() };

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

fn connecting_db(filepath: &str) {
    let db = JSONFileDatabase { file_path: filepath.to_owned() };
    match db.read_db() {
        Ok(data) => println!("✅ Loaded DB: {:?}", data),
        Err(err) => println!("❌ Failed to read DB: {}", err),
    }
}

fn main() -> Result<()> {
    seeding_db("db4.json")?;
    connecting_db("db4.json");
    Ok(())
}

// // อ่านก่อน
// match db.read_db() {
//     Ok(data) => println!("✅ Loaded DB: {:?}", data),
//     Err(err) => println!("❌ Failed to read DB: {}", err),
// }


// use serde::{Serialize, Deserialize};
// use serde_json;

// mod models;
// use models::{Epic, Story};

// mod db;

// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     name: String,
//     age: u8,
// }

// fn main() {
//     let st = Epic::new(
//         "aaa".to_owned(),
//         "ddd".to_owned()
//     );

//     let st_json = serde_json::to_string_pretty(&st).unwrap();
//     println!("{}", st_json);

//     let sto = Story::new(
//         "name".to_string(),
//         "description".to_string()
//     );

//     let sto_json = serde_json::to_string_pretty(&sto).unwrap();
//     println!("{}", sto_json);
// }

// use tempfile::NamedTempFile;
// use std::io::Write;

// fn main() -> std::io::Result<()> {
//     let mut file = NamedTempFile::new()?;
//     writeln!(file, "Hello, temp file!")?;
//     println!("Temp file path: {:?}", file.path());
//     Ok(())
// }