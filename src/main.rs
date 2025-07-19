mod models;

fn main() {
    let st = models::Epic {
        id: 1,
        name: "aaa".to_owned(),
        description: "ddd".to_owned(),
        status: models::Status::Closed
    };

    println!("{:?}", st);
}



    // println!("Hello, world!");
    // dbg!(models::Status::Closed);
    // println!(models::Status::Closed);

    // let result = models::Status::Closed;
    // println!("{:?}", result);