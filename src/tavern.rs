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

    fn check_name(hero: &Hero, name: &String) -> bool {
        let hero_name = hero.get_hero_name();
        if hero_name == *name {
            true
        } else {
            false
        }
        
    }

    pub fn kick_hero(&mut self, name: &str, kick_motive: &str) {
        
        //Este código es mucho más óptimo que el que había pensado
        //pide que se remueva algo y para ello dentro del remove de la lista
        // 1 - Lo va iterando
        // 2 - Se queda la posición
        // 3 - El predicado (con closures crea una comparativa al instante y devuelve incluso mensaje de error);
        //con lo que coge la posición del index exacto donde hay un caso positivo dentro de una sola línea
        //genio de idea
        self.people.remove(self.people.iter().position(|hero| hero.get_hero_name() == name).expect("Héroe no encontrado"));


        //funciona pero no es tan óptimo y gasta más recursos

        /* 
        let mut counter = 0;
        let mut exact_position = 0;
        for hero in self.people.iter_mut() {
            if Tavern::check_name(hero, &name) {
                exact_position = counter;
            } else {
                counter += 1;
            }
        }

        if exact_position >= 0 {
            self.people.remove(exact_position);
        }*/

        println!("\nEl héroe {} ha sido echado por: {}\n", name, kick_motive);
        
    }

    //Añadida capacidad de imprimir todo lo que contiene la taberna
    pub fn write_json_tavern(&self) {    
        std::fs::write("test.json", serde_json::to_string_pretty(&self.people).expect("Error"));
    }

}