use std::{fs::File};
use log::{info};
use postgres::{Client, NoTls};
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

pub fn send_vec_to_postgres(tavern: &Tavern) {
    tavern.heroes.iter().for_each(|hero| {
        info!("Hero {} sent to database", &hero.get_hero_name().to_uppercase());
        hero.send_to_postgres()
    });
}

pub fn create_table() {
    let mut client = Client::connect("host=localhost user=postgres password=contrasena dbname=rust", NoTls).expect("Connection Error");

    client.batch_execute("
    CREATE TABLE heroes (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        race    TEXT NOT NULL,
        weapon  TEXT NOT NULL,
        class   TEXT NOT NULL)").expect("Error creating table");
    info!("The table HEROES has been created!");
}