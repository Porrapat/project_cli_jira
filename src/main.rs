mod db;
mod models;
mod utility;
use anyhow::Result;
use crate::models::{DBState, Epic, Story, Status};
use crate::utility::{seeding_db, connecting_db};

use crate::db::{JSONFileDatabase, Database, JiraDatabase};

fn main() -> Result<()> {
    let db = JiraDatabase::new("db1.json".to_owned());

    // let epic1 = Epic {
    //     name: "Example Epic xxx".to_string(),
    //     description: "Testing epic xxx".to_string(),
    //     status: Status::Open,
    //     stories: vec![4,5],
    // };

    // db.create_epic(epic1);
    let x = db.create_story(Story {
        name: "".to_owned(),
        description: "".to_owned(),
        status: Status::Open,
    }, 122);
    // println!("HHHHHH");
    match x {
        Ok(id) => println!("Id is {id}"),
        Err(e) => println!("{}", e)
    }

    Ok(())
}

// let db = JSONFileDatabase { file_path: "./data/db.json".to_owned() };
// // let db = JSONFileDatabase { file_path: filepath.to_owned() };
// match db.read_db() {
//     Ok(data) => println!("✅ Loaded DB: {:?}", data),
//     Err(err) => println!("❌ Failed to read DB: {}", err),
// }

// db.
// todo!("Not IMP");
// seeding_db("db_temp.json")?;
// connecting_db("db_temp.json");
// clearscreen::clear().expect("failed to clear screen");

// use ellipse::Ellipse;

// let input = "🇩🇪🇬🇧🇮🇹🇫🇷";
// println!("{}", &input.truncate_ellipse(2));
// // assert_eq!(&input.truncate_ellipse(2), "🇩🇪🇬🇧...");