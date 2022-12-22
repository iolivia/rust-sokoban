use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

use crate::{
    components::{Box, BoxSpot, Position},
    resources::{Gameplay, GameplayState},
};

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    // Data
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay_state, positions, boxes, box_spots) = data;

        // get all boxes indexed by position
        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        // loop through all box spots and check if there is a corresponding
        // box at that position
        for (box_spot, position) in (&box_spots, &positions).join() {
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                if the_box.color == box_spot.color {
                    // continue
                } else {
                    // haven't won yet
                    return;
                }
            } else {
                gameplay_state.state = GameplayState::Playing;
                return;
            }
        }

        // If we made it this far, then all box spots have boxes on them, and the
        // game has been won
        gameplay_state.state = GameplayState::Won;
    }
}
