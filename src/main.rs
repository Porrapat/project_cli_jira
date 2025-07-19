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
