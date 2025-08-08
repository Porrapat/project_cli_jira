#![allow(unused)]

mod db;
mod models;
mod utility;
use anyhow::Result;
use crate::models::{DBState, Epic, Story, Status};
use crate::utility::{seeding_db, connecting_db};

use crate::db::{JSONFileDatabase, Database, JiraDatabase};

// mod db; // <-- ต้องเพิ่มบรรทัดนี้ก่อน เพื่อ import db.rs เข้ามา

use db::test_utils::MockDB;
use std::collections::HashMap;

fn main() -> Result<()> {

    let mock_db = MockDB::new();
    let result = mock_db.read_db(); // ได้ DBState ที่ว่างๆ

    let new_state = DBState {
        last_item_id: 42,
        epics: HashMap::new(),
        stories: HashMap::new(),
    };

    mock_db.write_db(&new_state); // เขียนสถานะใหม่เข้าไป
    // let db = JiraDatabase::new("db1.json".to_owned());
    // println!("Hello world!");
    Ok(())
}

