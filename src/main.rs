mod input;

use bevy::prelude::*;
use input::Input;
use std::{thread, time::Duration};
use structopt::StructOpt;

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

fn print_grid(grid_size: Res<InputSize>) {
    // clear terminal
    print!("\x1B[2J");
    println!(" __{}__", "__".repeat(grid_size.0));
    println!("|  {}  |", "__".repeat(grid_size.0));
    // world.lock().unwrap().print_grid();
    println!("| |{}| |", "__".repeat(grid_size.0));
    println!("|__{}__|", "__".repeat(grid_size.0));
    // world.lock().unwrap().print_tag_count()
}

// Resources
struct InputSize(usize);

fn main() {
    let Input {
        agents: n_of_agents,
        time: sleep_in_millis,
        size,
        moves,
        print_announce: announce_tag,
        disable_grid,
    } = Input::from_args();

    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(InputSize(size))
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .add_system(print_grid.system())
        .add_system(sleep2s.system())
        .run();
}
