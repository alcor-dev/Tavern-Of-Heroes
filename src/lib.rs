use std::{fs::File};
use tavern::Tavern;

pub mod heroes;
pub mod tavern;

pub fn read_json_tavern(path: &str) {
    let file = std::fs::read_to_string(path).expect("Error reading file");
    let final_json: Tavern = serde_json::from_str(&file).expect("Error deserializing JSON");

    println!("{:#?}", final_json);
}

pub fn check_file(path: &str) -> bool {
    let result = File::open(path);

    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}