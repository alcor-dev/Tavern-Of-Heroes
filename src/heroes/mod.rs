pub(crate) mod Heroes {

    #[derive(Debug)]
    pub enum Race {
        Human,
        Orc,
        Demon,
        Elf,
    }

    #[derive(Debug)]
    pub enum Weapon {
        Sword,
        Spear,
        Magic,
        Mace,
        Daggers,
        Hammer,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Class {
        Mage,
        Warrior,
        Rogue,
        Nechromancer,
    }

    #[derive(Debug)]
    pub struct Hero {
        name: String,
        race: Race,
        weapon: Weapon,
        class: Class,
        hp: u64,
        mana: u64,
    }

    fn check_class(&class: &Class) -> (u64, u64) {
        match &class {
            &Class::Mage => (250, 1000),
            &Class::Warrior => (1000, 100),
            &Class::Rogue => (500, 500),
            &Class::Nechromancer => (100, 1000),
        }
    }

    impl Hero {
        pub fn new(name: String, race: Race, weapon: Weapon, class: Class) -> Self {
            let (hp, mana) = check_class(&class);
            Self {
                name,
                race,
                weapon,
                class,
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
                _ => String::from("Another race"),
            };

            let weapon_txt = match &weapon {
                Weapon::Daggers => String::from("Daggers"),
                Weapon::Hammer => String::from("Hammer"),
                Weapon::Mace => String::from("Mace"),
                Weapon::Magic => String::from("Magic"),
                Weapon::Spear => String::from("Spear"),
                Weapon::Sword => String::from("Sword"),
                _ => String::from("Another weapon"),
            };

            let class_txt = match &class {
                Class::Mage => String::from("Mage"),
                Class::Nechromancer => String::from("Nechromancer"),
                Class::Rogue => String::from("Rogue"),
                Class::Warrior => String::from("Warrior"),
                _ => String::from("Another class"),
            };

            println!("Name: {}\nRace: {}\nWeapon: {}\nClass: {}\nHP: {}\nMana: {}", name, race_txt, weapon_txt, class_txt, hp, mana);

        }
    }
    
}