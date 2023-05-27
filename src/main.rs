use crate::heroes::*;
use crate::tavern::*;

mod heroes;
mod tavern;

fn main() {

    //Creador chorra de personajes en Rust
    let first_hero = Hero::new("Thrall", "orc", "hammer", "warrior");
    let second_hero = Hero::new("Drizzt", "elf", "daggers", "rogue");
    let third_hero = Hero::new("Althael", "demon", "magic", "nechromancer");

    println!("{:#?}", &first_hero);

    //Muestra la info del personaje elegido
    first_hero.describe();    

    //Creamos una taberna modificable
    let mut dark_tavern = Tavern::new("Dark Tavern");

    //Añadimos héroes o villanos
    dark_tavern.add(first_hero);
    dark_tavern.add(second_hero);
    dark_tavern.add(third_hero);

    //Pedimos que muestre los personajes con un formateo propio
    dark_tavern.show_heroes();

    //Echamos a uno de los héroes porque siempre gorronea (incluso incluimos motivos!)
    dark_tavern.kick_hero("Althael", "Gorronear y nunca pagar");

    //Mostramos que de verdad ha sido echado de la taberna
    dark_tavern.show_heroes();

    println!("{:#?}", dark_tavern);

    dark_tavern.write_json_tavern();

}