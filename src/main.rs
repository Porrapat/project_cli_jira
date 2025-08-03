mod db;
mod models;
mod utility;
use anyhow::Result;
use crate::utility::{seeding_db, connecting_db};

use crate::db::{JSONFileDatabase, Database, JiraDatabase};

fn main() -> Result<()> {

    let db = JSONFileDatabase { file_path: "./data/db.json".to_owned() };
    // let db = JSONFileDatabase { file_path: filepath.to_owned() };
    match db.read_db() {
        Ok(data) => println!("✅ Loaded DB: {:?}", data),
        Err(err) => println!("❌ Failed to read DB: {}", err),
    }

    // db.
    // todo!("Not IMP");
    // seeding_db("db_temp.json")?;
    // connecting_db("db_temp.json");
    // clearscreen::clear().expect("failed to clear screen");

    // use ellipse::Ellipse;

    // let input = "🇩🇪🇬🇧🇮🇹🇫🇷";
    // println!("{}", &input.truncate_ellipse(2));
    // // assert_eq!(&input.truncate_ellipse(2), "🇩🇪🇬🇧...");
    Ok(())
}