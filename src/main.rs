mod db;
mod models;
mod utility;

use anyhow::Result;

fn main() -> Result<()> {
    utility::seeding_db("db_temp.json")?;
    utility::connecting_db("db_temp.json");
    Ok(())
}