#![allow(unused)]

mod db;
mod models;
mod utility;
mod ui;

use anyhow::Result;
use crate::models::{DBState, Epic, Story, Status};
use crate::utility::{seeding_db, connecting_db};

use crate::db::{JSONFileDatabase, Database, JiraDatabase};

use db::test_utils::MockDB;
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("Hello world!");
    Ok(())
}

