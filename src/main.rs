mod db;
mod models;
mod utility;
use anyhow::Result;
use crate::utility::{seeding_db, connecting_db};

fn main() -> Result<()> {
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