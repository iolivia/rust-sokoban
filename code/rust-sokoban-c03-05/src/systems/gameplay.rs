use crate::components::*;
use hecs::World;

use std::collections::HashMap;

pub fn run_gameplay_state(world: &World) {
    // get all boxes indexed by position
    let mut query = world.query::<(&Position, &Box)>();
    let boxes_by_position: HashMap<(u8, u8), &Box> = query
        .iter()
        .map(|(_, t)| ((t.0.x, t.0.y), t.1))
        .collect::<HashMap<_, _>>();

    // loop through all box spots and check if there is a corresponding
    // box at that position
    let boxes_out_of_position: usize = world
        .query::<(&Position, &BoxSpot)>()
        .iter()
        .map(|(_, (position, box_spot))| {
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                if box_spot.colour == the_box.colour {
                    0
                } else {
                    1
                }
            } else {
                1
            }
        })
        .collect::<Vec<usize>>()
        .into_iter()
        .sum();

    // If we made it this far, then all box spots have boxes on them, and the
    // game has been won
    if boxes_out_of_position == 0 {
        let mut query = world.query::<&mut Gameplay>();
        let gameplay = query.iter().next().unwrap().1;
        gameplay.state = GameplayState::Won;
    }
}
