use std::{fs::File};
use log::{info, error};
use postgres::{Client, NoTls, Row, error};
use classes::{
    heroes,
    tavern::Tavern};

pub mod classes;

pub fn read_json_tavern(path: &str) {
    let file = std::fs::read_to_string(path).expect("Error reading file");
    let final_json: Tavern = serde_json::from_str(&file).expect("Error deserializing JSON");

    println!("{:#?}", final_json);
}

pub fn json_to_program(path: &str) -> Tavern {
    let file = std::fs::read_to_string(path).expect("Error reading file");
    let return_tavern: Tavern = serde_json::from_str(&file).expect("Error deserializing JSON");
    return_tavern
}

pub fn check_file(path: &str) -> bool {
    let result = File::open(path);

    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn send_to_database(tavern: &Tavern) {
    tavern.heroes.iter().for_each(|hero| {
        info!("Hero {} sent to database", &hero.get_hero_name().to_uppercase());
        hero.send_to_postgres();
    });
}

pub fn create_table() {
    let mut client = Client::connect("host=localhost user=postgres password=contrasena dbname=rust", NoTls).expect("Connection Error");

    client.batch_execute("
    CREATE TABLE IF NOT EXISTS heroes (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        race    TEXT NOT NULL,
        weapon  TEXT NOT NULL,
        class   TEXT NOT NULL)").expect("Error creating table");

    info!("The table HEROES has been created!");
}

pub fn drop_table() {

    let mut client = Client::connect("host=localhost user=postgres password=contrasena dbname=rust", NoTls).expect("Connection Error");
    
    client.execute("DROP TABLE heroes", &[]).expect("Error dropping table");

    info!("Deleting the table HEROES");    
}

pub fn read_everything() {
    
    let mut client = Client::connect("host=localhost user=postgres password=contrasena dbname=rust", NoTls).expect("Connection Error");

    let mut results: Vec<(i32, String, String, String, String)> = Vec::new();

    for row in client.query("SELECT * FROM heroes", &[]).expect("Error") {
        
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let race: String = row.get(2);
        let weapon: String = row.get(3);
        let class: String = row.get(4);

        info!("Found hero {} data! Reading from Database", &name.to_uppercase());
        results.push((id, name, race, weapon,class));
    };

    info!("Showing all the info contained in the results");
    println!("{:#?}", results);

}

pub fn search_in_table(name: &str) {
    
    let mut client = Client::connect("host=localhost user=postgres password=contrasena dbname=rust", NoTls).expect("Connection Error");

    let mut results: Vec<(i32, String, String, String, String)> = Vec::new();
    let mut counter :i32 = 1;
    

    for row in client.query("SELECT * FROM heroes WHERE name = $1", &[&name]).expect("Error") {
        
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let race: String = row.get(2);
        let weapon: String = row.get(3);
        let class: String = row.get(4);

        
        results.push((id, name, race, weapon,class));

        counter += 1;
    };

    info!("Found hero {}! | {} instance(s) | Retrieving data from database", &name.to_uppercase(), results.len());
    info!("Printing the search made with -> {}", &name.to_uppercase());

    if results.len() > 0 {
        println!("{:#?}", results);
    } else {
        info!("No instances of {} were found", &name.to_uppercase());
    }
    
}