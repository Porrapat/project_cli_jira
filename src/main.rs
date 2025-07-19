use serde::{Serialize, Deserialize};
use serde_json;

mod models;
use models::{Epic, Story};

// mod db;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let st = Epic::new(
        "aaa".to_owned(),
        "ddd".to_owned()
    );

    let st_json = serde_json::to_string_pretty(&st).unwrap();
    println!("{}", st_json);

    let sto = Story::new(
        "name".to_string(),
        "description".to_string()
    );

    let sto_json = serde_json::to_string_pretty(&sto).unwrap();
    println!("{}", sto_json);
}

