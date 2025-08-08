
    // db.update_story_status(3, Status::Closed);
    // db.update_story_status(3, Status::InProgress);
    // db.update_epic_status(1, Status::InProgress);
    // db.update_epic_status(1, Status::Closed);
    // db.create_story(story, epic_id)

    // let x = db.create_story(Story {
    //     name: "TTTT".to_owned(),
    //     description: "PPPP".to_owned(),
    //     status: Status::Open,
    // }, 100);

    // match x {
    //     Ok(story_id) => { println!("Story id {story_id}"); },
    //     Err(e) => { println!("Err {e}"); }
    // }

    // let epic1 = Epic {
    //     name: "Example Epic xxx".to_string(),
    //     description: "Testing epic xxx".to_string(),
    //     status: Status::Open,
    //     stories: vec![4,5],
    // };

    // let x = db.create_epic(epic1);

    // match x {
    //     Ok(epic_id) => { println!("Epic id {epic_id} created"); },
    //     Err(e) => { println!("Err {e}"); }
    // }

    // let x: Result<u32> = Ok(11);
    // if let Ok(epic_id) = x {
    //     println!("Epic deleted with id: {epic_id}");
    //     db.delete_epic(epic_id);
    // } else {
    //     println!("Failed to create epic");
    // }

    // db.delete_epic(epic_id);

    // db.delete_story(1, 8);

    // let x = db.create_story(Story {
    //     name: "".to_owned(),
    //     description: "".to_owned(),
    //     status: Status::Open,
    // }, 122);
    // // println!("HHHHHH");
    // match x {
    //     Ok(id) => println!("Id is {id}"),
    //     Err(e) => println!("{}", e)
    // }

    // db.delete_epic(123);
    // db.delete_story(111,222);
    // db.update_epic_status(12, Status::Closed);
    // db.update_story_status(1234, Status::InProgress);



    // let db = JSONFileDatabase { file_path: "./data/db.json".to_owned() };
// // let db = JSONFileDatabase { file_path: filepath.to_owned() };
// match db.read_db() {
//     Ok(data) => println!("✅ Loaded DB: {:?}", data),
//     Err(err) => println!("❌ Failed to read DB: {}", err),
// }

// db.
// todo!("Not IMP");
// seeding_db("db_temp.json")?;
// connecting_db("db_temp.json");
// clearscreen::clear().expect("failed to clear screen");

// use ellipse::Ellipse;

// let input = "🇩🇪🇬🇧🇮🇹🇫🇷";
// println!("{}", &input.truncate_ellipse(2));
// // assert_eq!(&input.truncate_ellipse(2), "🇩🇪🇬🇧...");