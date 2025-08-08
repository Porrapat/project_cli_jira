#![allow(unused)]

use std::fs;
use serde_json;

use anyhow::Result;
use anyhow::anyhow;

use crate::models::{DBState, Epic, Story, Status};

pub struct JiraDatabase {
    database: Box<dyn Database>
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase { file_path })
        }
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
        let new_id = last_id + 1;
        
        parsed.last_item_id = new_id;
        parsed.stories.insert(new_id, story);
        parsed.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("could not find epic in database!"))?.stories.push(new_id);
        
        self.database.write_db(&parsed)?;
        Ok(epic_id)
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

#[cfg(test)]
mod tests {
    use super::*;

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
            // TODO: fix this error by deriving the appropriate traits for DBState
            assert_eq!(read_result, state);
        }
    }
}