#![allow(unused)]

mod db;
mod models;
mod utility;
use anyhow::Result;
use crate::models::{DBState, Epic, Story, Status};
use crate::utility::{seeding_db, connecting_db};

use crate::db::{JSONFileDatabase, Database, JiraDatabase};

fn main() -> Result<()> {
    let db = JiraDatabase::new("db1.json".to_owned());
    println!("Hello world!");
    Ok(())
}

