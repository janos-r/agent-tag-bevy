mod input;
mod resources;

use bevy::prelude::*;
use rand::Rng;
use resources::*;
use std::{thread, time::Duration};

struct Agent;

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    Normal,
    Tagged,
    UnTaggable,
}
struct Position(usize, usize);

fn add_agents(
    mut commands: Commands,
    n_of_agents: Res<InputAgents>,
    mut rng: ResMut<Random>,
    world_size: Res<InputSize>,
) {
    for _ in 0..n_of_agents.0 {
        commands
            .spawn()
            .insert(Agent)
            .insert(Status::Normal)
            .insert(Position(
                rng.0.gen_range(0..world_size.0),
                rng.0.gen_range(0..world_size.0),
            ));
    }
}

fn sleep2s(sleep_in_millis: Res<InputTime>) {
    thread::sleep(Duration::from_millis(sleep_in_millis.0));
}

fn update_grid(
    agents: Query<(&Position, &Status), With<Agent>>,
    mut grid: ResMut<Grid>,
    grid_size: Res<InputSize>,
) {
    // init empty grid
    let mut new_grid: Grid = Grid(
        (0..grid_size.0)
            .map(|_| (0..grid_size.0).map(|_| None).collect())
            .collect(),
    );
    // populate grid with agents
    agents.iter().for_each(|(position, status)| {
        new_grid.0[position.1][position.0] = Some(status.clone());
    });
    grid.0 = new_grid.0;
}

fn print_grid(grid: Res<Grid>, grid_size: Res<InputSize>, tag_count: Res<TagCount>) {
    // clear terminal
    print!("\x1B[2J");
    println!(" __{}__", "__".repeat(grid_size.0));
    println!("|  {}  |", "__".repeat(grid_size.0));

    grid.0.iter().for_each(|row| {
        let line: String = row
            .iter()
            .map(|field| match field {
                Some(Status::Tagged) => "😈️",
                Some(Status::UnTaggable) => "😀️",
                Some(Status::Normal) => "🐏️",
                None => "  ",
            })
            .collect();
        println!("| |{}| |", line)
    });

    println!("| |{}| |", "__".repeat(grid_size.0));
    println!("|__{}__|", "__".repeat(grid_size.0));
    println!("Total count of exchanged tags: {}", tag_count.0)
}

fn main() {
    static UPDATE_GRID: &str = "update_grid";
    static PRINT_GRID: &str = "print_grid";
    static SLEEP: &str = "sleep";
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(LoadResources)
        .add_startup_system(add_agents.system())
        // Run every tick
        .add_system(update_grid.system().label(UPDATE_GRID))
        .add_system(print_grid.system().label(PRINT_GRID).after(UPDATE_GRID))
        .add_system(sleep2s.system().label(SLEEP).after(PRINT_GRID))
        .run();
}
