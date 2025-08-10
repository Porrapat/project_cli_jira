#![allow(unused)]

mod db;
mod models;
mod utility;
mod ui;
mod io_utils;
mod navigator;

use anyhow::Result;
use crate::models::{DBState, Epic, Story, Status};
use crate::utility::{seeding_db, connecting_db};

use crate::db::{JSONFileDatabase, Database, JiraDatabase};

use db::test_utils::MockDB;
use std::collections::HashMap;

use crate::ui::StoryDetail;
use crate::ui::EpicDetail;
use crate::ui::HomePage;
use crate::ui::Page;

use crate::io_utils::get_user_input;

fn main() -> Result<()> {

    println!("สวัสดีโปรแกรมของผม");
    println!("======================");
    // let s = get_user_input();
    // println!("{s}");
    // use std::rc::Rc;

    // let db = Rc::new(JiraDatabase::new("db1.json".to_owned())); // หรือใส่ mock data ไปเลย
    // let home_page = HomePage { db };

    // home_page.draw_page()?;
    // // let ret = home_page.handle_input("c");
    // let ret = home_page.handle_input("c")?;

    // println!("{ret:?}");
    // match re {

    // }

    // let epic_detail_page = EpicDetail { epic_id: 1, db };

    // epic_detail_page.draw_page()?;
    // let ret = epic_detail_page.handle_input("u")?;

    // println!("{ret:?}");

    // let story_detail_page = StoryDetail { story_id: 2, epic_id: 1, db };

    // story_detail_page.draw_page()?;
    // let ret = story_detail_page.handle_input("d")?;

    // println!("{ret:?}");

    Ok(())
}

