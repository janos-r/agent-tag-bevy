use super::input::Input;
use super::Status;
use bevy::prelude::{AppBuilder, Plugin};
use rand::{rngs::SmallRng, SeedableRng};
use structopt::StructOpt;

// Resources
pub struct InputAgents(pub usize);
pub struct InputTime(pub u64);
pub struct InputSize(pub usize);
pub struct InputAnnounceTag(pub bool);

// Todo:
pub struct InputMoves(pub usize);
pub struct InputDisableGrid(pub bool);

#[derive(Default)]
pub struct TagCount(pub u32);
pub struct Random(pub SmallRng);
pub struct Grid(pub Vec<Vec<Option<Status>>>);

pub struct LoadResources;
impl Plugin for LoadResources {
    fn build(&self, app: &mut AppBuilder) {
        let Input {
            agents: n_of_agents,
            time: sleep_in_millis,
            size,
            moves,
            print_announce: announce_tag,
            disable_grid,
        } = Input::from_args();
        app.insert_resource(InputAgents(n_of_agents))
            .insert_resource(InputTime(sleep_in_millis))
            .insert_resource(InputSize(size))
            .insert_resource(InputMoves(moves))
            .insert_resource(InputAnnounceTag(announce_tag))
            .insert_resource(InputDisableGrid(disable_grid))
            .insert_resource(Random(SmallRng::from_entropy()))
            .insert_resource(Grid(Vec::with_capacity(size)))
            .init_resource::<TagCount>();
    }
}
