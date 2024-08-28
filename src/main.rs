use std::collections::HashMap;

mod utils;

fn main() {
    let level_string = utils::level::decode_level();
    let level = utils::level::parse_level(level_string);
    println!("{:?}", level);
}

#[derive(Debug)]
struct Level {
    data: HashMap<String, String>,
    colors: Vec<HashMap<String, String>>,
    objects: Vec<HashMap<String, String>>,
}
