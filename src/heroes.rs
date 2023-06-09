use serde::{Serialize, Deserialize};
use postgres::{Client, NoTls};
use log::info;
 
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Race {
    Human,
    Orc,
    Demon,
    Elf,
    Dwarf,
    Goblin,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Weapon {
    Sword,
    Spear,
    Scepter,
    Magic,
    Mace,
    Daggers,
    Hammer,
    Hands,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Class {
    Mage,
    Warrior,
    Rogue,
    Nechromancer,
    Fighter,
    Karate,
    Healer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    name: String,
    race: Race,
    weapon: Weapon,
    class: Class,
    hp: u64,
    mana: u64,
}

fn check_class(class: &str) -> (u64, u64) {
    match class {
        "healer" => (200, 3000),
        "mage" => (250, 1000),
        "warrior" => (1000, 100),
        "rogue" => (500, 500),
        "nechromancer" => (100, 1000),
        "karate" => (5000, 0),
        "fighter" => (2000, 50),
        &_ => (0,0),
    }
}

pub trait Nameable {
    fn nameable(&self) -> &str;
}

impl Hero {
    pub fn new(name: &str, race: &str, weapon: &str, class: &str) -> Self {
        
        //usamos el to_lowercase, para convertir todo en minúsculas
        //para acabar usando el as_str para convertirlo a &str
        let enum_race = match race.to_lowercase().as_str() {
            "human" => Race::Human, 
            "orc" => Race::Orc,
            "demon" => Race::Demon,
            "elf" => Race::Elf,
            "dwarf" => Race::Dwarf,
            "goblin" => Race::Goblin,
            &_ => todo!(),
        };

        let enum_weapon = match weapon.to_lowercase().as_str() {
            "sword" => Weapon::Sword,
            "spear" => Weapon::Spear,
            "magic" => Weapon::Magic,
            "mace" => Weapon::Mace,
            "daggers" => Weapon::Daggers,
            "hammer" => Weapon::Hammer,
            "hands" => Weapon::Hands,
            "scepter" => Weapon::Scepter,
            &_ => todo!(),  
        };

        let enum_class = match class.to_lowercase().as_str() {
            "healer" => Class::Healer,
            "mage" => Class::Mage,
            "warrior" => Class::Warrior,
            "fighter" => Class::Fighter,
            "karate" => Class::Karate,
            "nechromancer" => Class::Nechromancer,
            "rogue" => Class::Rogue,
            &_ => todo!(),
        };

        let (hp, mana) = check_class(&class);

        //log de creación del personaje
        info!("The hero {} has been created", &name);

        Self {
            name: String::from(name),
            race: enum_race,
            weapon: enum_weapon,
            class: enum_class,
            hp,
            mana,
        }
        
    }

    pub fn describe(&self) {
        let hero_test = self;
        let Hero{name, race, weapon, class, hp, mana} = hero_test;

        let race_txt = match &race {
            Race::Demon => String::from("Demon"),
            Race::Elf => String::from("Elf"),
            Race::Human => String::from("Human"),
            Race::Orc => String::from("Orc"),
            Race::Dwarf => String::from("Dwarf"),
            Race::Goblin => String::from("Goblin")
        };

        let weapon_txt = match &weapon {
            Weapon::Daggers => String::from("Daggers"),
            Weapon::Hands => String::from("Hands"),
            Weapon::Hammer => String::from("Hammer"),
            Weapon::Mace => String::from("Mace"),
            Weapon::Magic => String::from("Magic"),
            Weapon::Spear => String::from("Spear"),
            Weapon::Sword => String::from("Sword"),
            Weapon::Scepter => String::from("Scepter")
            
        };

        let class_txt = match &class {
            Class::Mage => String::from("Mage"),
            Class::Nechromancer => String::from("Nechromancer"),
            Class::Rogue => String::from("Rogue"),
            Class::Warrior => String::from("Warrior"),
            Class::Fighter => String::from("Fighter"),
            Class::Karate => String::from("Karate Master"),
            Class::Healer => String::from("Healer")
        };

        println!("Name: {}\nRace: {}\nWeapon: {}\nClass: {}\nHP: {}\nMana: {}\n\n", name, race_txt, weapon_txt, class_txt, hp, mana);

    }

    pub fn get_hero_name(&self) -> String {
        let Hero{name, ..} = self;
        String::from(name)
    }

        
    pub fn send_to_postgres(&self) {
        let mut client = Client::connect("host=localhost user=postgres password=contrasena dbname=rust", NoTls).expect("Connection Error");

        let race_txt = match self.race {
            Race::Demon => String::from("Demon"),
            Race::Elf => String::from("Elf"),
            Race::Human => String::from("Human"),
            Race::Orc => String::from("Orc"),
            Race::Dwarf => String::from("Dwarf"),
            Race::Goblin => String::from("Goblin")
        };

        let weapon_txt = match self.weapon {
            Weapon::Daggers => String::from("Daggers"),
            Weapon::Hands => String::from("Hands"),
            Weapon::Hammer => String::from("Hammer"),
            Weapon::Mace => String::from("Mace"),
            Weapon::Magic => String::from("Magic"),
            Weapon::Spear => String::from("Spear"),
            Weapon::Sword => String::from("Sword"),
            Weapon::Scepter => String::from("Scepter")
            
        };

        let class_txt = match self.class {
            Class::Mage => String::from("Mage"),
            Class::Nechromancer => String::from("Nechromancer"),
            Class::Rogue => String::from("Rogue"),
            Class::Warrior => String::from("Warrior"),
            Class::Fighter => String::from("Fighter"),
            Class::Karate => String::from("Karate Master"),
            Class::Healer => String::from("Healer")
        };

        //El uso de una query con $ y números sucesivos ayuda a evitar inyecciones de SQL con malas intenciones
        client.execute(
            "INSERT INTO heroes (name, race, weapon, class) VALUES ($1, $2, $3, $4)",
            &[&self.name, &race_txt, &weapon_txt, &class_txt]
        ).expect("Data not inserted");
    }

}

//Añadimos el trait nameable a la struct Hero
impl Nameable for Hero {
    fn nameable(&self) -> &str {
        self.name.as_str()
    }
}

pub fn say_name<T: Nameable>(item: T) {
    println!("Name of the hero is: {}", item.nameable())
}