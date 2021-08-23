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
    thread::sleep(Duration::from_millis(sleep_in_millis.0));
}

fn main() {
    static UPDATE_GRID: &str = "update_grid";
    static PRINT_GRID: &str = "print_grid";
    static MOVE_AGENTS: &str = "move_agents";
    static TAG: &str = "tag";
    static SLEEP: &str = "sleep";
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
        .run();
}
