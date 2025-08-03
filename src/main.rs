mod db;
mod models;
mod utility;
use anyhow::Result;
use crate::utility::{seeding_db, connecting_db};

fn main() -> Result<()> {
    seeding_db("db_temp.json")?;
    connecting_db("db_temp.json");
    Ok(())
}