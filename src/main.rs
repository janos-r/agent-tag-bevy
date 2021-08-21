mod input;

use bevy::prelude::*;
use input::Input;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{thread, time::Duration};
use structopt::StructOpt;

struct Agent;

#[derive(Clone, PartialEq, Debug)]
enum Status {
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

// Resources
struct InputAgents(usize);
struct InputTime(u64);
struct InputSize(usize);
struct InputMoves(usize);
struct InputAnnounceTag(bool);
struct InputDisableGrid(bool);

#[derive(Default)]
struct TagCount(u32);
struct Random(SmallRng);
struct Grid(Vec<Vec<Option<Status>>>);

fn main() {
    let Input {
        agents: n_of_agents,
        time: sleep_in_millis,
        size,
        moves,
        print_announce: announce_tag,
        disable_grid,
    } = Input::from_args();

    static UPDATE_GRID: &str = "update_grid";
    static PRINT_GRID: &str = "print_grid";
    static SLEEP: &str = "sleep";
    App::build()
        .add_plugins(MinimalPlugins)
        .add_stage_after(
            CoreStage::Update,
            UPDATE_GRID,
            SystemStage::single_threaded(),
        )
        .add_stage_after(UPDATE_GRID, PRINT_GRID, SystemStage::single_threaded())
        .add_stage_after(PRINT_GRID, SLEEP, SystemStage::single_threaded())
        .insert_resource(InputAgents(n_of_agents))
        .insert_resource(InputTime(sleep_in_millis))
        .insert_resource(InputSize(size))
        .insert_resource(InputMoves(moves))
        .insert_resource(InputAnnounceTag(announce_tag))
        .insert_resource(InputDisableGrid(disable_grid))
        .insert_resource(Random(SmallRng::from_entropy()))
        .insert_resource(Grid(Vec::with_capacity(size)))
        .init_resource::<TagCount>()
        .add_startup_system(add_agents.system())
        .add_system_to_stage(UPDATE_GRID, update_grid.system())
        .add_system_to_stage(PRINT_GRID, print_grid.system())
        .add_system_to_stage(SLEEP, sleep2s.system())
        .run();
}
