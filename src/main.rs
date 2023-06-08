use character_creator::{heroes::{Hero, say_name}, tavern::Tavern};

fn main() {

    env_logger::init();
    //Creador chorra de personajes en Rust
    let first_hero = Hero::new("Thrall", "orc", "hammer", "warrior");
    let second_hero = Hero::new("Drizzt", "elf", "daggers", "rogue");
    let third_hero = Hero::new("Althael", "demon", "magic", "nechromancer");
    let fourth_hero = Hero::new("Myst", "demon", "hands", "karate");
    let fifth_hero = Hero::new("Elizabeth", "elf", "scepter", "healer");

    println!("{:#?}", &first_hero);

    //Muestra la info del personaje elegido
    first_hero.describe();    

    //Creamos una taberna modificable
    let mut dark_tavern = Tavern::new("Dark Tavern");

    //Añadimos héroes o villanos
    dark_tavern.add(first_hero);
    dark_tavern.add(second_hero);
    dark_tavern.add(third_hero);
    dark_tavern.add(fourth_hero);
    dark_tavern.add(fifth_hero);

    //Creador automático de personajes
    dark_tavern.create_characters();

    //Pedimos que muestre los personajes con un formateo propio
    dark_tavern.show_heroes();

    //Echamos a uno de los héroes porque siempre gorronea (incluso incluimos motivos!)
    dark_tavern.kick_hero("Anthonio", "Gorronear y nunca pagar");

    //Mostramos que de verdad ha sido echado de la taberna
    dark_tavern.show_heroes();

    //uso de función genérica para llamar a todos los structs con el trait Nameable [ver heroes.rs]
    //update: usando clone, podemos hacer una copia que se consuma en lugar del original
    say_name(dark_tavern.heroes[0].clone());

    println!("{:#?}", dark_tavern);

    //Creamos un archivo con los héroes de la taberna
    dark_tavern.write_json_tavern().expect("Error");

}