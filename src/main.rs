use heroes::Heroes::{Race, Weapon, Class};

use crate::heroes::Heroes::Hero;

pub mod heroes;

fn main() {
    //Creador chorra de personajes en Rust
    let first_hero = Hero::new(String::from("Thrall"), Race::Orc, Weapon::Hammer, Class::Warrior);
    let second_hero = Hero::new(String::from("Drizzt"), Race::Elf, Weapon::Daggers, Class::Rogue);
    println!("{:?}", &first_hero);

    first_hero.describe();

    let mut tabern = Vec::new();

    tabern.push(first_hero);
    tabern.push(second_hero);

    for hero in tabern.iter_mut() {
        println!("{:?}", hero)
    }

    println!("{:?}", &tabern[0]);
    println!("{:?}", &tabern[1]);
}

