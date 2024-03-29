use std::io;
use serde::{Serialize, Deserialize};
use log::{info, error};

use crate::heroes::{Hero};

pub struct KickMotive {
    name: String,
    kick_motive: String,
}

impl KickMotive {
    pub fn new(name: &str, kick_motive: &str) -> Self {
        Self{
            name: String::from(name),
            kick_motive: String::from(kick_motive),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_kick_motive(&self) -> &str {
        &self.kick_motive
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Tavern {
    name: String,
    pub heroes: Vec<Hero>,
}

impl Tavern {
    pub fn new(name: &str) -> Self {

        info!("Created the tavern -> {}", &name.to_uppercase());
        Self {
            name: String::from(name),
            heroes: Vec::new(),
        }
    }

    pub fn add(&mut self, hero: Hero) {        
        info!("Added the hero {:#?}", &hero);
        self.heroes.push(hero);
    }

    pub fn show_heroes(&mut self) {
        for hero in self.heroes.iter_mut() {
            //hero.describe();
            println!("{:#?}", hero)
        }
    }

    pub fn kick_hero(&mut self, name: &str, kick_motive: &str) {
        
        //Este código es mucho más óptimo que el que había pensado
        //pide que se remueva algo y para ello dentro del remove de la lista
        // 1 - Avanza sobre el código
        // 2 - Se queda la posición
        // 3 - El predicado (con closures crea una comparativa al instante y devuelve incluso mensaje de error)
        //con lo que coge la posición del index exacto donde hay un caso positivo dentro de una sola línea
        //genio de idea
        //update: usando un Option, devolvemos la posición con un Some(usize) y en caso contrario con un None, evitando panics
        //update 2: usamos una función para que la lectura sea aún más sencilla
        info!("Searching for -> {}", &name.to_uppercase());
        let find_hero: Option<usize> = self.find_hero(name);

        //Usamos un match para devolver un resultado u otro dependiendo de si encuentra algo o no, pero evitando el
        //código haciendo PANIC
        match find_hero {
            Some(usize) => {
                info!("SUCCESS! Hero {} was found! Proceeding to kicking it out!", &name.to_uppercase());
                self.heroes.remove(usize);
                println!("\nHero {} kicked due to: {}\n", name, kick_motive);
                },
            None => {
                error!("Error, hero {} couldn't be found", &name.to_uppercase());
                println!("There's not a hero with that name");
                }
        };
    }

    fn find_hero(&self, name: &str) -> Option<usize> {
        //let hero = self.heroes.iter().position(|hero| hero.get_hero_name() == name);
        let find_hero = self.heroes.iter().position(|hero| hero.get_hero_name() == name);
        find_hero
    }

    //Añadida capacidad de imprimir todo lo que contiene la taberna
    pub fn write_json_tavern(&self)  -> Result<(), io::Error>{
        let filename = "dark_tavern.json";    
        let read = std::fs::write(&filename, serde_json::to_string_pretty(&self)?)?;
        //al usar ? hacemos que Rust automáticamente gestione el procesado de los errores, en este caso, tanto serde_jason como write deben llevar uno cada uno
        //en caso contrario, debería usar
        /*match read {
            Ok(_) => println!("\nwriting of {} is successful", filename.to_uppercase()),
            Err(_) => println!("\nwriting has failed"),
        }*/
        Ok(read)
    }

    //Permite automatizar en gran medida la creación de personajes y actualización de la taberna
    pub fn create_characters(&mut self) {

        let mut counter: u32 = 1;
    
        println!("Introduce number of heroes to be made: ");
        
        let mut number_heroes = String::new();
        io::stdin().read_line(&mut number_heroes).expect("Error");
    
        let number_heroes = number_heroes.trim().parse::<u32>().expect("Error doing parsing");
        info!("The number of heroes to be made is: {}", &number_heroes);
    
        loop {

            if number_heroes == 0 { break};
    
            println!("\n\nHero number: [{}]", counter);
    
            println!("Introduce name of the character: ");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Error reading NAME");
    
            println!("Introduce race of the character: ");
            let mut race = String::new();
            io::stdin().read_line(&mut race).expect("Error reading RACE");
    
            println!("Introduce weapon of the character: ");
            let mut weapon = String::new();
            io::stdin().read_line(&mut weapon).expect("Error reading WEAPON");
    
            println!("Introduce class of the character: ");
            let mut class = String::new();
            io::stdin().read_line(&mut class).expect("Error reading CLASS");

            let new_hero = Hero::new(name.trim(), race.trim(), weapon.trim(), class.trim());
            self.heroes.push(new_hero);
            
            if counter >= number_heroes || number_heroes == counter { break };
            counter += 1;

        };   
    }    
}