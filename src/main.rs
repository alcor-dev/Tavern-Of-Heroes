use heroes::Heroes::{Race, Weapon, Class};

use crate::heroes::Heroes::Hero;
use crate::tavern::tavern::Tavern;

pub mod heroes;
pub mod tavern;

fn main() {

    //Creador chorra de personajes en Rust
    let first_hero = Hero::new(String::from("Thrall"), Race::Orc, Weapon::Hammer, Class::Warrior);
    let second_hero = Hero::new(String::from("Drizzt"), Race::Elf, Weapon::Daggers, Class::Rogue);
    let third_hero = Hero::new(String::from("Althael"), Race::Demon, Weapon::Magic, Class::Nechromancer);
    println!("{:?}", &first_hero);

    //Muestra la info del personaje elegido
    first_hero.describe();    

    //Creamos una taberna modificable
    let mut dark_tavern = Tavern::new(String::from("Dark Tavern"));

    //Añadimos héroes o villanos
    dark_tavern.add(first_hero);
    dark_tavern.add(second_hero);
    dark_tavern.add(third_hero);

    //Pedimos que muestre los personajes con un formateo propio
    dark_tavern.show_heroes();

    //Echamos a uno de los héroes porque siempre gorronea (incluso incluimos motivos!)
    dark_tavern.kick_hero(String::from("Althael"), String::from("Gorronear y nunca pagar"));

    //Mostramos que de verdad ha sido echado de la taberna
    dark_tavern.show_heroes();

    println!("{:?}", dark_tavern);
    
}