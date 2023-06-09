use character_creator::
    {heroes::{Hero, say_name}, 
    tavern::{Tavern}, read_json_tavern, check_file, send_vec_to_postgres, create_table};
use log::{info, error, LevelFilter};
use env_logger::{filter, Builder};

fn main() {

    Builder::new().filter(None, LevelFilter::Info).init();

    let path: Vec<String> = std::env::args().collect();
    let arg = path[1].as_str();
    
    //nos saltamos 0 porque sería el propio comando principal y pasamos al argumento recolectado
    //println!("{}", &path[1]);

    if check_file(&arg) {

        info!("File with the name {} has been found!", &arg.to_uppercase());
        println!("The file {} has been found", &arg.to_uppercase());
        read_json_tavern(arg);

    } else {

        error!("The file with the name {} couldn't be found",&arg.to_uppercase());
        println!("The original file {} couldn't be found so there will be another one created", &arg.to_uppercase());
        
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

        //Creamos unos métodos que nos permitan salvar la información 
        //en una base de datos, en este caso: POSTGRES
        //primero creando la tabla
        /*create_table();*/

        //Después guardando los personajes dentro de la tabla mediante queries
        send_vec_to_postgres(&dark_tavern);
        
        //Creador automático de personajes
        dark_tavern.create_characters();

        //Pedimos que muestre los personajes con un formateo propio
        dark_tavern.show_heroes();

        //Echamos a uno de los héroes porque siempre gorronea (incluso incluimos motivos!)
        dark_tavern.kick_hero("Althael", "Gorronear y nunca pagar");

        //Mostramos que de verdad ha sido echado de la taberna
        dark_tavern.show_heroes();

        //uso de función genérica para llamar a todos los structs con el trait Nameable [ver heroes.rs]
        //update: usando clone, podemos hacer una copia que se consuma en lugar del original
        say_name(dark_tavern.heroes[0].clone());

        println!("{:#?}", dark_tavern);

        //Creamos un archivo con los héroes de la taberna
        dark_tavern.write_json_tavern().expect("Error");
    }
}