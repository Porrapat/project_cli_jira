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
use crate::io_utils::wait_for_key_press;

use std::rc::Rc;
use navigator::*;

fn main() -> Result<()> {

    // let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    // let db = Rc::new(JiraDatabase::new("./db1.json".to_owned()));
    let db = Rc::new(JiraDatabase::new_sqlite("./data/jira.db".to_owned())?);
    let mut navigator = Navigator::new(Rc::clone(&db));

    // clearscreen::clear().unwrap();
    // println!("====== START Program =====");
    // wait_for_key_press();
    // clearscreen::clear().unwrap();

    // println!("====== Help =====");
    // wait_for_key_press();
    // clearscreen::clear().unwrap();

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            };

            let user_input = get_user_input();

            match page.handle_input(user_input.trim()) {
                Err(error) => {
                    println!(
                        "Error getting user input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("Error handling processing user input: {}\nPress any key to continue...", error);
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}
