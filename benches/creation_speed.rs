use criterion::{black_box, criterion_group, criterion_main, Criterion};

use character_creator::{heroes::*, tavern::Tavern};


pub fn creation_benchmark_iter(c: &mut Criterion) {
    let mut tavern = Tavern::new("Bench Tavern");

    let first_hero = Hero::new("Thrall", "orc", "hammer", "warrior");
    let second_hero = Hero::new("Drizzt", "elf", "daggers", "rogue");
    let third_hero = Hero::new("Althael", "demon", "magic", "nechromancer");
    let fourth_hero = Hero::new("Myst", "demon", "hands", "karate");
    let fifth_hero = Hero::new("Elizabeth", "elf", "scepter", "healer");

    let heroes = vec![first_hero, second_hero, third_hero, fourth_hero, fifth_hero];
    
    c.bench_function("creation iter", |b| b.iter( || {
        for hero in heroes.iter() {
            let hero = hero.clone();
            tavern.add(hero);
        }
    } ));
}

pub fn creation_benchmark_manually(c: &mut Criterion) {
    let mut tavern = Tavern::new("Bench Tavern");

    let first_hero = Hero::new("Thrall", "orc", "hammer", "warrior");
    let second_hero = Hero::new("Drizzt", "elf", "daggers", "rogue");
    let third_hero = Hero::new("Althael", "demon", "magic", "nechromancer");
    let fourth_hero = Hero::new("Myst", "demon", "hands", "karate");
    let fifth_hero = Hero::new("Elizabeth", "elf", "scepter", "healer");
    
    c.bench_function("creation manual", |b| b.iter( || {
        tavern.add(first_hero.clone());
        tavern.add(second_hero.clone());
        tavern.add(third_hero.clone());
        tavern.add(fourth_hero.clone());
        tavern.add(fifth_hero.clone());
    } ));
}

criterion_group!(benches, creation_benchmark_iter, creation_benchmark_manually);
criterion_main!(benches);