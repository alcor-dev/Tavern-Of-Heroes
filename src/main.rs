use std::rc::Rc;

use character_creator::
    {
        heroes::{Hero, say_name}, 
        tavern::{Tavern, self}, read_json_tavern, check_file, create_table, read_everything, search_in_table, send_to_database, drop_table, json_to_program
    };
use cursive::{Cursive, views::{Dialog, TextView, SelectView, LinearLayout, Button, DummyView, EditView, ListView, TextArea}, CursiveExt, align::{HAlign, Align}, view::{Scrollable, Resizable, Nameable}, event::Key};
use log::{info, error, LevelFilter};
use env_logger::{Builder};

fn main() {

    //Generación del logger donde no tiene limitacionees y el filtro ha sido configurado para mostrar
    //los avisos de menor nivel también y comprobar que funciona
    Builder::new().filter(None, LevelFilter::Info).init();

    let mut siv = Cursive::default();

    siv.add_global_callback( Key::Esc ,|s| s.quit());;
    

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
        .child(Button::new("Add new hero", add_hero))
        .child(Button::new("Remove hero", not_implemented))
        .child(DummyView)
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

fn add_hero(siv: &mut Cursive) {
    siv.pop_layer();    
    siv.add_layer(Dialog::new().title("HERO CREATOR").content(TextView::new("This is the default test")).dismiss_button("Ok"));
    siv.add_layer(Dialog::new().title("Hero Data Input")
        .content(ListView::new()
            .child(
                "Name: ",
                EditView::new().with_name("name")
            )
            .child(
                "Race: ",
                EditView::new().with_name("race")
            )
            .child(
                "Weapon: ",
                EditView::new().with_name("weapon")
            )
            .child(
                "Class: ",
                EditView::new().with_name("class")
            )
            ,
        ).button("Ok", |s| {
            let name = s.call_on_name("name", |t: &mut EditView| t.get_content());
            let race = s.call_on_name("race", |t: &mut EditView| t.get_content());
            let weapon = s.call_on_name("weapon", |t: &mut EditView| t.get_content());
            let class = s.call_on_name("class", |t: &mut EditView| t.get_content());
            
            let final_hero = Hero::new(&name.unwrap_or_default(), &race.unwrap_or_default(), &weapon.unwrap_or_default(), &class.unwrap_or_default());

            show_final_data(s, final_hero);
        }))
        ;


}

fn test(s: &mut Cursive) {
    s.add_layer(Dialog::info("Some important data!").title("DATA FROM HEROES"));
}

fn show_final_data(s: &mut Cursive, hero: Hero) {
    let hp = &hero.get_hp();
    s.pop_layer();
    s.add_layer(Dialog::new().title("DATA HERO CREATED!")
        .content(ListView::new()
            .child(
                "Name: ",
                TextView::new(&hero.get_hero_name())
            )
            .child(
                "Race: ",
                TextView::new(&hero.get_race())
            )
            .child(
                "Weapon: ",
                TextView::new(&hero.get_weapon())
            )
            .child(
                "Class: ",
                TextView::new(&hero.get_class())
            )
            .child(
                "HP: ",
                TextView::new(&hero.get_hp().to_string())
            )
            .child(
                "Mana: ",
                TextView::new(&hero.get_mana().to_string())
            )
            ,
    )
    .button("Next", move |s|{
        save_data(s, hero.clone())
    }))
}

fn save_data(siv: &mut Cursive, hero: Hero) {
    let mut tavern = json_to_program("dark_tavern.json");
    tavern.add(hero);
    tavern.write_json_tavern();

    siv.pop_layer();
    siv.add_layer(Dialog::new().title("DATA SAVED!").content(TextView::new("The hero you just created has been saved!")).button("Back", show_next));
    
}

fn show_name(siv: &mut Cursive, name: &str) {
    if name.is_empty() {
        siv.add_layer(Dialog::info("Please enter a name!"));
    } else {
        let content = format!("Hello {name}");
        siv.pop_layer();
        siv.add_layer(Dialog::around(TextView::new(content)).title("WARNING").dismiss_button("Quit"))
    }
}

fn show_race(siv: &mut Cursive, race: &str) {
    if race.is_empty() {
        siv.add_layer(Dialog::info("Please enter a race!"));
    } else {
        let content = format!("Hello {race}");
        siv.pop_layer();
        siv.add_layer(Dialog::around(TextView::new(content)).title("RACE WARNING").dismiss_button("Quit"))
    }
}

fn not_implemented(siv: &mut Cursive) {
    siv.add_layer(Dialog::around(TextView::new("Not implemented yet").content("Feature not implemented yet")).dismiss_button("Back"));
}