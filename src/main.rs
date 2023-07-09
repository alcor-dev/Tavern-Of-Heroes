use character_creator::
    {
        heroes::{Hero, say_name}, 
        tavern::{Tavern}, read_json_tavern, check_file, create_table, read_everything, search_in_table, send_to_database, drop_table, json_to_program
    };
use cursive::{Cursive, views::{Dialog, TextView, SelectView, LinearLayout, Button, DummyView}, CursiveExt, align::HAlign, view::{Scrollable, Resizable}};
use log::{info, error, LevelFilter};
use env_logger::{Builder};

fn main() {

    //Generación del logger donde no tiene limitacionees y el filtro ha sido configurado para mostrar
    //los avisos de menor nivel también y comprobar que funciona
    Builder::new().filter(None, LevelFilter::Info).init();

    let mut siv = Cursive::default();
    

    //Recoge los argumentos usados al iniciar el programa y los usa para configurar a qué archivo JSON acceder
    let path: Vec<String> = std::env::args().collect();
    let arg = path[1].as_str();
    
    //nos saltamos 0 porque sería el propio comando principal y pasamos al argumento recolectado

    if check_file(&arg) {

        info!("File with the name {} has been found!", &arg.to_uppercase());
        println!("The file {} has been found", &arg.to_uppercase());
        read_json_tavern(arg);
        let tavern_vec = json_to_program(arg);
        siv.add_layer(Dialog::around(TextView::new("Meet your heroes")).title("Dark Tavern").button("Next", show_next));

    } else {

        error!("The file with the name {} couldn't be found",&arg.to_uppercase());
        println!("The original file {} couldn't be found so there will be another one created", &arg.to_uppercase());
        
        //Creador chorra de personajes en Rust
        let first_hero = Hero::new("Thrall", "orc", "hammer", "warrior");
        let second_hero = Hero::new("Drizzt", "elf", "daggers", "rogue");
        let third_hero = Hero::new("Althael", "demon", "magic", "nechromancer");
        let fourth_hero = Hero::new("Myst", "demon", "hands", "karate");
        let fifth_hero = Hero::new("Elizabeth", "elf", "scepter", "healer");

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
        //EDIT: ahora no crashea pues tiene una cláusula en la query que si otra tabla con el
        //mismo nombre aparece, no haga nada
        create_table();

        //Después guardando los personajes dentro de la tabla mediante queries
        //send_to_database(&dark_tavern);

        //Leer de base de datos
        read_everything();

        //Busca un resultado determinado
        search_in_table("Myst");

        //Creador automático de personajes
        dark_tavern.create_characters();

        //Pedimos que muestre los personajes con un formateo propio
        //dark_tavern.show_heroes();

        //Echamos a uno de los héroes porque siempre gorronea (incluso incluimos motivos!)
        dark_tavern.kick_hero("Althael", "Gorronear y nunca pagar");

        //Mostramos que de verdad ha sido echado de la taberna
        //dark_tavern.show_heroes();

        //uso de función genérica para llamar a todos los structs con el trait Nameable [ver heroes.rs]
        //update: usando clone, podemos hacer una copia que se consuma en lugar del original
        //say_name(dark_tavern.heroes[0].clone());

        //println!("{:#?}", dark_tavern);

        //Creamos un archivo con los héroes de la taberna
        dark_tavern.write_json_tavern().expect("Error");

        //Destruyendo la tabla necesaria
        //Manteniendo esto activado junto con la creación de tablas se produce un efecto muy tipo
        //drop-create en bootspring al configurar todo
        /*drop_table();*/
        
    }

    siv.run();

}

fn show_next(siv: &mut Cursive) {
    let data = json_to_program("dark_tavern.json");
    let mut select: SelectView<Hero> = SelectView::new().h_align(HAlign::Center).autojump();
    let buttons = LinearLayout::vertical()
        // .child(Button::new("Add new", add_hero))
        .child(Button::new("Quit", Cursive::quit));
    for hero in data.heroes.iter() {
        select.add_item(&hero.get_hero_name(), hero.clone());
    }

    select.set_on_submit(show_hero);

    siv.pop_layer();
    siv.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(select)
        .child(DummyView)
        .child(buttons))
        .title("Heroes"));
}

fn show_hero(siv: &mut Cursive, hero: &Hero) {
    let text = format!("Name: {}\nRace: {}\nWeapon: {}\nClass: {}\nHp: {}\nMana: {}", hero.get_hero_name(), hero.get_race(), hero.get_weapon(), hero.get_class(), hero.get_hp(), hero.get_mana());
    siv.add_layer(Dialog::around(TextView::new(text)).button("Back", show_next).title(hero.get_hero_name()));

}