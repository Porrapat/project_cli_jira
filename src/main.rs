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

use crate::ui::HomePage;
use crate::ui::Page;

fn main() -> Result<()> {
    use std::rc::Rc;

    let db = Rc::new(JiraDatabase::new("db1.json".to_owned())); // หรือใส่ mock data ไปเลย
    let home_page = HomePage { db };

    home_page.draw_page()?;

    Ok(())
}

