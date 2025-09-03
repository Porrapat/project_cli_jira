#![allow(unused)]

use std::fs;
use serde_json;
use std::collections::HashMap;

use anyhow::Result;
use anyhow::anyhow;
use rusqlite::{Connection, params};

use crate::models::{DBState, Epic, Story, Status};

pub struct JiraDatabase {
    pub database: Box<dyn Database>
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase { file_path })
        }
    }

    pub fn new_sqlite(file_path: String) -> Result<Self> {
        Ok(Self {
            database: Box::new(SQLiteDatabase::new(file_path)?)
        })
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut parsed = self.database.read_db()?;
    
        let last_id = parsed.last_item_id;
        let new_id = last_id + 1;
        
        parsed.last_item_id = new_id;
        parsed.epics.insert(new_id, epic);
    
        self.database.write_db(&parsed)?;
        Ok(new_id)
    }
    
    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut parsed = self.database.read_db()?;
    
        let last_id = parsed.last_item_id;
        let new_story_id = last_id + 1;
        
        parsed.last_item_id = new_story_id;
        parsed.stories.insert(new_story_id, story);
        parsed.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("could not find epic in database!"))?.stories.push(new_story_id);
    
        self.database.write_db(&parsed)?;
        Ok(new_story_id)
    }
    
    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut parsed = self.database.read_db()?;
    
        for story_id in &parsed.epics.get(&epic_id).ok_or_else(|| anyhow!("could not find epic in database!"))?.stories {
            parsed.stories.remove(story_id);
        }
        
        parsed.epics.remove(&epic_id);
    
        self.database.write_db(&parsed)?;
        Ok(())
    }
    
    pub fn delete_story(&self,epic_id: u32, story_id: u32) -> Result<()> {
        let mut parsed = self.database.read_db()?;
    
        let epic = parsed.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("could not find epic in database!"))?;
    
        let story_index = epic.stories.iter().position(|id| id == &story_id).ok_or_else(|| anyhow!("story id not found in epic stories vector"))?;
        epic.stories.remove(story_index);
    
        parsed.stories.remove(&story_id);
    
        self.database.write_db(&parsed)?;
        Ok(())
    }
    
    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut parsed = self.database.read_db()?;
        if let Some(epic) = parsed.epics.get_mut(&epic_id) {
            epic.status = status;
            self.database.write_db(&parsed)?;
            Ok(())
        } else {
            Err(anyhow!("Epic with id {} not found", epic_id))
        }
    }
    
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut parsed = self.database.read_db()?;

        if let Some(story) = parsed.stories.get_mut(&story_id) {
            story.status = status;
            self.database.write_db(&parsed)?;
            Ok(())
        } else {
            Err(anyhow!("Story with id {} not found", story_id))
        }
    }
}

pub trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;
    
    pub struct MockDB {
        last_written_state: RefCell<DBState>
    }

    impl MockDB {
        pub fn new() -> Self {
            Self { last_written_state: RefCell::new(DBState { last_item_id: 0, epics: HashMap::new(), stories: HashMap::new() }) }
        }    
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

pub struct JSONFileDatabase {
    pub file_path: String
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let content = fs::read_to_string(&self.file_path)?;
        let db_state: DBState = serde_json::from_str(&content)?;
        Ok(db_state)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let content = serde_json::to_string_pretty(db_state)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}

pub struct SQLiteDatabase {
    pub file_path: String
}

impl SQLiteDatabase {
    pub fn new(file_path: String) -> Result<Self> {
        let db = Self { file_path };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = Connection::open(&self.file_path)?;
        
        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS epics (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS stories (
                id INTEGER PRIMARY KEY,
                epic_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                status TEXT NOT NULL,
                FOREIGN KEY(epic_id) REFERENCES epics(id)
            )",
            [],
        )?;

        // Initialize last_item_id if it doesn't exist
        conn.execute(
            "INSERT OR IGNORE INTO metadata (key, value) VALUES ('last_item_id', 0)",
            [],
        )?;

        Ok(())
    }

    fn status_to_string(status: &Status) -> String {
        match status {
            Status::Open => "Open".to_string(),
            Status::InProgress => "InProgress".to_string(),
            Status::Resolved => "Resolved".to_string(),
            Status::Closed => "Closed".to_string(),
        }
    }

    fn string_to_status(status_str: &str) -> Result<Status> {
        match status_str {
            "Open" => Ok(Status::Open),
            "InProgress" => Ok(Status::InProgress),
            "Resolved" => Ok(Status::Resolved),
            "Closed" => Ok(Status::Closed),
            _ => Err(anyhow!("Invalid status: {}", status_str)),
        }
    }
}

impl Database for SQLiteDatabase {
    fn read_db(&self) -> Result<DBState> {
        let conn = Connection::open(&self.file_path)?;
        
        // Get last_item_id
        let last_item_id: u32 = conn.query_row(
            "SELECT value FROM metadata WHERE key = 'last_item_id'",
            [],
            |row| row.get(0)
        )?;

        // Get all epics
        let mut epics = HashMap::new();
        let mut stmt = conn.prepare("SELECT id, name, description, status FROM epics")?;
        let epic_rows = stmt.query_map([], |row| {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            let description: String = row.get(2)?;
            let status_str: String = row.get(3)?;
            let status = Self::string_to_status(&status_str).map_err(|e| rusqlite::Error::InvalidColumnType(0, "status".to_string(), rusqlite::types::Type::Text))?;
            
            Ok((id, Epic { name, description, status, stories: Vec::new() }))
        })?;

        for epic_row in epic_rows {
            let (id, epic) = epic_row?;
            epics.insert(id, epic);
        }

        // Get all stories and associate them with epics
        let mut stories = HashMap::new();
        let mut stmt = conn.prepare("SELECT id, epic_id, name, description, status FROM stories")?;
        let story_rows = stmt.query_map([], |row| {
            let id: u32 = row.get(0)?;
            let epic_id: u32 = row.get(1)?;
            let name: String = row.get(2)?;
            let description: String = row.get(3)?;
            let status_str: String = row.get(4)?;
            let status = Self::string_to_status(&status_str).map_err(|e| rusqlite::Error::InvalidColumnType(0, "status".to_string(), rusqlite::types::Type::Text))?;
            
            Ok((id, epic_id, Story { name, description, status }))
        })?;

        for story_row in story_rows {
            let (story_id, epic_id, story) = story_row?;
            stories.insert(story_id, story);
            
            // Add story ID to the epic's stories vector
            if let Some(epic) = epics.get_mut(&epic_id) {
                epic.stories.push(story_id);
            }
        }

        Ok(DBState {
            last_item_id,
            epics,
            stories,
        })
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let conn = Connection::open(&self.file_path)?;
        
        // Start transaction
        let tx = conn.unchecked_transaction()?;

        // Update last_item_id
        tx.execute(
            "UPDATE metadata SET value = ?1 WHERE key = 'last_item_id'",
            params![db_state.last_item_id],
        )?;

        // Clear existing data
        tx.execute("DELETE FROM stories", [])?;
        tx.execute("DELETE FROM epics", [])?;

        // Insert epics
        for (epic_id, epic) in &db_state.epics {
            tx.execute(
                "INSERT INTO epics (id, name, description, status) VALUES (?1, ?2, ?3, ?4)",
                params![epic_id, epic.name, epic.description, Self::status_to_string(&epic.status)],
            )?;
        }

        // Insert stories
        for (story_id, story) in &db_state.stories {
            // Find which epic this story belongs to
            let epic_id = db_state.epics.iter()
                .find(|(_, epic)| epic.stories.contains(story_id))
                .map(|(id, _)| *id)
                .ok_or_else(|| anyhow!("Story {} not found in any epic", story_id))?;

            tx.execute(
                "INSERT INTO stories (id, epic_id, name, description, status) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![story_id, epic_id, story.name, story.description, Self::status_to_string(&story.status)],
            )?;
        }

        // Commit transaction
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_utils::MockDB;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic.clone());
        
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story.clone(), epic_id);
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&id), true);
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);
        
        let story_id = result.unwrap();

        let non_existent_epic_id = 999;
        
        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let non_existent_story_id = 999;
        
        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id), false);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_story_status_should_work() {
        let db = JiraDatabase { database: Box::new(MockDB::new()) };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.stories.get(&story_id).unwrap().status, Status::Closed);
    }

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { file_path: "INVALID_PATH".to_owned() };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}
