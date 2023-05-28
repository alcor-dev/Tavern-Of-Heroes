use std::io;

use crate::heroes::*;

#[derive(Debug)]
pub struct Tavern {
    name: String,
    pub people: Vec<Hero>,
}

impl Tavern {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            people: Vec::new(),
        }
    }

    pub fn add(&mut self, hero: Hero) {
        self.people.push(hero);
    }

    pub fn show_heroes(&mut self) {
        for hero in self.people.iter_mut() {
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
        let find_hero: Option<usize> = self.people.iter().position(|hero| hero.get_hero_name() == name);

        //Usamos un match para devolver un resultado u otro dependiendo de si encuentra algo o no, pero evitando el
        //código haciendo PANIC
        match find_hero {
            Some(usize) => {
                self.people.remove(usize);
                println!("\nEl héroe {} ha sido echado por: {}\n", name, kick_motive);
                },
            None => println!("No existe un héroe con ese nombre"),
        };
    }

    fn find_hero(&self, name: &str) -> Option<usize> {
        //let hero = self.people.iter().position(|hero| hero.get_hero_name() == name);
        let find_hero = self.people.iter().position(|hero| hero.get_hero_name() == name);
        find_hero
    }

    //Añadida capacidad de imprimir todo lo que contiene la taberna
    pub fn write_json_tavern(&self) {    
        std::fs::write("dark_tavern.json", serde_json::to_string_pretty(&self.people).expect("Error"));
    }

    pub fn create_characters(&mut self) {

        let mut counter: u32 = 1;
    
        println!("Introduce number of heroes to be made: ");
        let mut number_heroes = String::new();
        io::stdin().read_line(&mut number_heroes).expect("Error");
    
        let number_heroes = number_heroes.trim().parse::<u32>().expect("Error doing parsing");
    
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

            let new_hero = (Hero::new(name.trim(), race.trim(), weapon.trim(), class.trim()));
            self.people.push(new_hero);
            
            if counter >= number_heroes || number_heroes == counter { break };
            counter += 1;

        };   
    }
}