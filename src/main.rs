use bevy::prelude::*;
use std::{thread, time::Duration};

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
    println!("");
}

fn sleep2s() {
    thread::sleep(Duration::from_millis(2000));
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .add_system(sleep2s.system())
        .run();
}
