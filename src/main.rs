use std::rc::Rc;

use tavern_of_heroes::
    {
        classes::heroes::{Hero, say_name}, 
        classes::tavern::{Tavern, self, KickMotive}, read_json_tavern, check_file, create_table, read_everything, search_in_table, send_to_database, drop_table, json_to_program
    };
use cursive::{Cursive, views::{Dialog, TextView, SelectView, LinearLayout, Button, DummyView, EditView, ListView, TextArea}, CursiveExt, align::{HAlign, Align}, view::{Scrollable, Resizable, Nameable, Finder}, event::Key};
use log::{info, error, LevelFilter};
use env_logger::{Builder};

fn main() {

    //Generación del logger donde no tiene limitacionees y el filtro ha sido configurado para mostrar
    //los avisos de menor nivel también y comprobar que funciona
    Builder::new().filter(None, LevelFilter::Info).init();

    let mut siv = Cursive::default();

    siv.add_global_callback( Key::Esc ,|s| s.quit());;


    //Recoge los argumentos usados al iniciar el programa y los usa para configurar a qué archivo JSON acceder
    //update: ya no se usa para buscar el nombre del archivo pero mostramos los parámetros por curiosidad
    let path: Vec<String> = std::env::args().collect();

    if path.len() > 1 {
        let mut counter: u8 = 0;

        for word in path {
            println!("Parameter[{}]: {}", counter, word.as_str());
            counter += 1;
        }
    } else {
        println!("No parameters have been added");
    }
    

    //Hardcodeamos el argumento porque creo que así encaja mejor con la idea original
    let arg = "dark_tavern.json";
    
    //Nos saltamos 0 porque sería el propio comando principal y pasamos al argumento recolectado

    if check_file(&arg) {

        info!("File with the name {} has been found!", &arg.to_uppercase());
        println!("The file {} has been found", &arg.to_uppercase());
        read_json_tavern(arg);
        siv.add_layer(Dialog::around(TextView::new("Meet your heroes")).title("Dark Tavern").button("Next", main_menu));

    } else {

        error!("The file with the name {} couldn't be found",&arg.to_uppercase());
        println!("The original file {} couldn't be found so there will be another one created", &arg.to_uppercase());
        
        //Creador chorra de personajes en Rust
 
        let heroes = vec! [
            Hero::new("Thrall", "orc", "hammer", "warrior"),
            Hero::new("Drizzt", "elf", "daggers", "rogue"),
            Hero::new("Althael", "demon", "magic", "nechromancer"),
            Hero::new("Myst", "demon", "hands", "karate"),
            Hero::new("Elizabeth", "elf", "scepter", "healer")
        ];

        //Muestra la info del personaje elegido
        heroes[0].describe();        

        //Creamos una taberna modificable
        let mut dark_tavern = Tavern::new("Dark Tavern");

        //Añadimos héroes o villanos

        for hero in heroes {
            dark_tavern.add(hero);

        }

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


//Con Cursive creamos un menú principal donde mostramos los datos recopilados del JSON
//además de mostrar las diferentes opciones para interactuar con el programa
fn main_menu(siv: &mut Cursive) {
    let data = json_to_program("dark_tavern.json");

    //Creamos una instancia de SelectView con el tipo Hero para así poder luego
    //seleccionar los héroes cargados dentro de una lista interactiva
    let mut select: SelectView<Hero> = SelectView::new().h_align(HAlign::Center).autojump();
    let buttons = LinearLayout::vertical()
        // .child(Button::new("Add new", add_hero))
        .child(Button::new("Add new hero", add_hero))
        .child(Button::new("Remove hero", delete_hero))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));
    
    //Introducimos la lista de héroes de Tavern y los añadimos al SelectView
    for hero in data.heroes.iter() {
        select.add_item(&hero.get_hero_name(), hero.clone());
    }

    //Hacemos que cuando se seleccione alguna de las cosas de la lista del SelectView
    //esta active otra capa de la aplicación y muestre lo seleccionado
    select.set_on_submit(show_hero);

    //Eliminamos con pop_layer() la capa creada anteriormente para evitar sobrecarga visual
    siv.pop_layer();
    //Con esta nueva capa, creamos un layout con 3 partes, una donde irán los seleccionables
    //de una lista que hemos creado con el SelectView, luego un hueco para dejar un poco de
    //espacio y finalmente los botones para interactuar
    siv.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(select)
        .child(DummyView)
        .child(buttons))
        .title("Heroes"));
}

//En esta capa mostramos la información del héroe seleccionado dentro del SelectView
fn show_hero(siv: &mut Cursive, hero: &Hero) {
    let text = format!("Name: {}\nRace: {}\nWeapon: {}\nClass: {}\nHp: {}\nMana: {}", hero.get_hero_name(), hero.get_race(), hero.get_weapon(), hero.get_class(), hero.get_hp(), hero.get_mana());
    siv.add_layer(Dialog::around(TextView::new(text)).button("Back", main_menu).title(hero.get_hero_name()));

}

//Creamos una nueva ventana donde mostramos los datos que queremos introducir en el personaje nuevo
fn add_hero(siv: &mut Cursive) {
    siv.pop_layer();    
    siv.add_layer(Dialog::new().title("HERO CREATOR").content(TextView::new("This is the default test")).dismiss_button("Ok"));
    //Creamos una capa con una ventana de diálogo que contendrá una ListView donde podremos añadir varios
    //items de forma seguida muy cómodamente
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

            //Aquí hacemos que el botón nos lleve a una nueva parte del programa llevando consigo la información que nos interesa
            show_final_data(s, final_hero);
        }))
        ;


}

//Mostramos la información que ha sido creada mediante la introducción en el paso anterior
fn show_final_data(s: &mut Cursive, hero: Hero) {
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
    tavern.write_json_tavern().expect("Error writing JSON data");

    siv.pop_layer();
    siv.add_layer(Dialog::new().title("DATA SAVED!").content(TextView::new("The hero you just created has been saved!")).button("Back", main_menu));
    
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

//Pequeño placeholder fácil de usar para cuando se introduzca una opción y no haya código temporalmente
fn not_implemented(siv: &mut Cursive) {
    siv.add_layer(Dialog::around(TextView::new("Not implemented yet").content("Feature not implemented yet")).dismiss_button("Back"));
}

fn delete_hero(siv: &mut Cursive) {
    siv.pop_layer();
    siv.add_layer(Dialog::new().title("Deleting hero")
                .content(
                    ListView::new()
                        .child(
                            "Issue a ban -> ",
                            TextView::new("Add the data to justify the kick\n")
                        )
                        .child(
                            "",
                            TextView::new(" ")
                        )
                        .child(
                            "Name: ",
                            EditView::new().with_name("name")
                        )
                        .child(
                            "Kick motive: ",
                            EditView::new().with_name("kick_motive")
                        )
                        ,
                ).button("Continue", |s| {
                    let name = s.call_on_name("name", |t: &mut EditView| t.get_content());
                    let kick_motive = s.call_on_name("kick_motive", |t: &mut EditView| t.get_content());

                    let ban = KickMotive::new(&name.unwrap_or_default(), &kick_motive.unwrap_or_default());

                    show_delete(s, ban)
                }));
}

fn show_delete(siv: &mut Cursive, km: KickMotive) {
    let mut tavern = json_to_program("dark_tavern.json");
    tavern.kick_hero(km.get_name(), km.get_kick_motive());
    tavern.write_json_tavern().expect("Error writing JSON data");

    let kick_final = format!("Hero {} has been kicked due to {}", km.get_name(), km.get_kick_motive());

    siv.pop_layer();
    siv.add_layer(Dialog::around(TextView::new(kick_final)).title("DELETING HEROE").button("Ok", main_menu));   
}