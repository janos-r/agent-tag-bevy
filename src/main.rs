mod entities;
mod grid;
mod input;
mod resources;

use bevy::prelude::*;
use entities::*;
use grid::*;
use resources::*;
use std::{thread, time::Duration};

fn sleep2s(sleep_in_millis: Res<InputTime>) {
    if sleep_in_millis.0 > 0 {
        thread::sleep(Duration::from_millis(sleep_in_millis.0));
    }
}

// Terminate if the desired number of moves ticked - for benchmarking
fn exit(
    mut move_count: ResMut<MoveCount>,
    terminate_in_move: Res<InputMoves>,
    disable_grid: Res<InputDisableGrid>,
    tag_count: Res<TagCount>,
) {
    if move_count.0 == terminate_in_move.0 {
        if disable_grid.0 {
            println!("Total count of exchanged tags: {}", tag_count.0)
        }
        std::process::exit(0)
    } else {
        move_count.0 += 1;
    }
}

fn main() {
    static UPDATE_GRID: &str = "update_grid";
    static PRINT_GRID: &str = "print_grid";
    static MOVE_AGENTS: &str = "move_agents";
    static TAG: &str = "tag";
    static SLEEP: &str = "sleep";
    static EXIT: &str = "exit";
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(LoadResources)
        .add_startup_system(add_agents.system())
        // Run every tick
        .add_system(update_grid.system().label(UPDATE_GRID))
        .add_system(print_grid.system().label(PRINT_GRID).after(UPDATE_GRID))
        .add_system(move_agents.system().label(MOVE_AGENTS).after(PRINT_GRID))
        .add_system(tag.system().label(TAG).after(MOVE_AGENTS))
        .add_system(sleep2s.system().label(SLEEP).after(TAG))
        .add_system(exit.system().label(EXIT).after(SLEEP))
        .run();
}
