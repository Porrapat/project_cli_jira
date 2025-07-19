use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let user = User {
        name: "เปา".to_string(),
        age: 37,
    };

    // Serialize Rust -> JSON
    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json); // {"name":"เปา","age":37}

    // Deserialize JSON -> Rust
    let decoded: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", decoded);
}

// mod models;
// use models::{Epic, Story};
// fn main() {
//     let st = Epic::new(
//         "aaa".to_owned(),
//         "ddd".to_owned()
//     );
    
//     println!("{:?}", st.description);
//     let sto = Story::new(
//         "name".to_string(),
//         "description".to_string()
//     );
//     println!("{:?}", sto.description);
// }
