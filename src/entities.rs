use super::*;
use rand::Rng;

// ===========
// Agent utils
// ===========
pub struct Agent;

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    Normal,
    Tagged,
    UnTaggable,
}
#[derive(PartialEq)]
pub struct Position(pub usize, pub usize);

pub fn add_agents(
    mut commands: Commands,
    n_of_agents: Res<InputAgents>,
    mut rng: ResMut<Random>,
    world_size: Res<InputSize>,
) {
    // -1 tagged
    for _ in 0..n_of_agents.0 - 1 {
        commands
            .spawn()
            .insert(Agent)
            .insert(Status::Normal)
            .insert(Position(
                rng.0.gen_range(0..world_size.0),
                rng.0.gen_range(0..world_size.0),
            ));
    }

    // Make one tagged
    commands
        .spawn()
        .insert(Agent)
        .insert(Status::Tagged)
        .insert(Position(
            rng.0.gen_range(0..world_size.0),
            rng.0.gen_range(0..world_size.0),
        ));
}

fn position_add(current_axis_position: usize, grid_size: usize) -> usize {
    (current_axis_position + 1) % (grid_size - 1)
}

fn position_sub(current_axis_position: usize, grid_size: usize) -> usize {
    if current_axis_position == 0 {
        // last index
        grid_size - 1
    } else {
        current_axis_position - 1
    }
}

pub fn move_agents(
    agents: Query<&mut Position, With<Agent>>,
    mut rng: ResMut<Random>,
    grid_size: Res<InputSize>,
) {
    agents.for_each_mut(|mut position| {
        let direction = rng.0.gen_range(0..4);
        match direction {
            // on edges - pop out on the other side
            0 => position.0 = position_add(position.0, grid_size.0),
            1 => position.0 = position_sub(position.0, grid_size.0),
            2 => position.1 = position_add(position.1, grid_size.0),
            _ => position.1 = position_sub(position.1, grid_size.0),
        }
    });
}

pub fn tag(
    mut agents: Query<(&mut Status, &Position, Entity), With<Agent>>,
    mut tag_count: ResMut<TagCount>,
    grid_size: Res<InputSize>,
    announce_tag: Res<InputAnnounceTag>,
) {
    let mut origin: Option<(Entity, [Position; 4])> = None;
    let mut target: Option<Entity> = None;

    // First iteration - get tagged entity and its neighbors
    for (status, position, entity) in agents.iter_mut() {
        if *status == Status::Tagged {
            let neighbors = [
                Position(position_add(position.0, grid_size.0), position.1),
                Position(position_sub(position.0, grid_size.0), position.1),
                Position(position.0, position_add(position.1, grid_size.0)),
                Position(position.0, position_sub(position.1, grid_size.0)),
            ];
            origin = Some((entity, neighbors));
            break;
        };
    }

    // Second iteration - find an entity on a neighboring position
    if let Some((_, neighbors)) = &origin {
        for (status, position, entity) in agents.iter_mut() {
            if neighbors.iter().any(|neighbor_position| {
                neighbor_position == position && *status != Status::UnTaggable
            }) {
                if announce_tag.0 {
                    println!("!!!! FOUND NEIGHBOR !!!!");
                }
                target = Some(entity);
                // found first neighbor - no need to continue searching
                break;
            }
        }
    }

    // If found neighbor (target) -> tag
    if let Some(target) = target {
        if let Some((origin, _)) = origin {
            // raise tag count
            tag_count.0 += 1;

            // reset all statuses
            agents.for_each_mut(|(mut status, _, _)| *status = Status::Normal);

            // set tag and untaggable
            if let Ok((mut status, _, _)) = agents.get_mut(target) {
                *status = Status::Tagged
            }
            if let Ok((mut status, _, _)) = agents.get_mut(origin) {
                *status = Status::UnTaggable
            }
        }
    }
}
