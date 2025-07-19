mod models;
use models::{Epic, Story};
fn main() {
    let st = Epic::new(
        "aaa".to_owned(),
        "ddd".to_owned()
    );
    
    let sto = Story::new(
        "name".to_string(),
        "description".to_string()
    );
    println!("{:?}", sto.description);
}


    // let st = models::Epic {
    //     // id: 1,
    //     name: "aaa".to_owned(),
    //     description: "ddd".to_owned(),
    //     status: models::Status::Closed
    // };
    // {
    //     // id: 1,
    //     name: "aaa".to_owned(),
    //     description: "ddd".to_owned(),
    //     status: models::Status::Closed
    // };
    // println!("Hello, world!");
    // dbg!(models::Status::Closed);
    // println!(models::Status::Closed);

    // let result = models::Status::Closed;
    // println!("{:?}", result);