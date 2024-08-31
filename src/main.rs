use std::collections::HashMap;

mod utils;

fn main() {
    let level_string = utils::level::decode_level();
    let level = utils::level::parse_level(level_string);
    let test = utils::level::stringify_level(level);
    let finished = utils::level::encode_level(test);
    println!("{}", finished);
    println!("done");
}

#[derive(Debug)]
struct Level {
    data: HashMap<String, String>,
    colors: Vec<HashMap<String, String>>,
    objects: Vec<HashMap<String, String>>,
}
