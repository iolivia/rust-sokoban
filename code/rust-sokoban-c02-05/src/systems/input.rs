use ggez::{input::keyboard::KeyCode, Context};
use hecs::{Entity, World};

use std::collections::HashMap;

use crate::components::*;
use crate::constants::*;

// ANCHOR: run_input_begin
pub fn run_input(world: &World, context: &mut Context) {
    let mut to_move: Vec<(Entity, KeyCode)> = Vec::new();

    // ANCHOR_END: run_input_begin
    // get all the movables and immovables
    let mov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Movable)>()
        .iter()
        .map(|t| ((t.1 .0.x, t.1 .0.y), t.0))
        .collect::<HashMap<_, _>>();
    let immov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Immovable)>()
        .iter()
        .map(|t| ((t.1 .0.x, t.1 .0.y), t.0))
        .collect::<HashMap<_, _>>();

    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        if context.keyboard.is_key_repeated() {
            continue;
        }

        // Now iterate through current position to the end of the map
        // on the correct axis and check what needs to move.
        let key = if context.keyboard.is_key_just_pressed(KeyCode::Up) {
            KeyCode::Up
        } else if context.keyboard.is_key_just_pressed(KeyCode::Down) {
            KeyCode::Down
        } else if context.keyboard.is_key_just_pressed(KeyCode::Left) {
            KeyCode::Left
        } else if context.keyboard.is_key_just_pressed(KeyCode::Right) {
            KeyCode::Right
        } else {
            continue;
        };

        let (start, end, is_x) = match key {
            KeyCode::Up => (position.y, 0, false),
            KeyCode::Down => (position.y, MAP_HEIGHT - 1, false),
            KeyCode::Left => (position.x, 0, true),
            KeyCode::Right => (position.x, MAP_WIDTH - 1, true),
            _ => continue,
        };

        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        for x_or_y in range {
            let pos = if is_x {
                (x_or_y, position.y)
            } else {
                (position.x, x_or_y)
            };

            // find a movable
            // if it exists, we try to move it and continue
            // if it doesn't exist, we continue and try to find an immovable instead
            match mov.get(&pos) {
                Some(entity) => to_move.push((*entity, key)),
                None => {
                    // find an immovable
                    // if it exists, we need to stop and not move anything
                    // if it doesn't exist, we stop because we found a gap
                    match immov.get(&pos) {
                        Some(_id) => to_move.clear(),
                        None => break,
                    }
                }
            }
        }
    }
    // ANCHOR: run_input_update_moves
    // Update gameplay moves
    if !to_move.is_empty() {
        let mut query = world.query::<&mut Gameplay>();
        let gameplay = query.iter().next().unwrap().1;
        gameplay.moves_count += 1;
    }

    // Now actually move what needs to be moved
    for (entity, key) in to_move {
        let mut position = world.get::<&mut Position>(entity).unwrap();

        match key {
            KeyCode::Up => position.y -= 1,
            KeyCode::Down => position.y += 1,
            KeyCode::Left => position.x -= 1,
            KeyCode::Right => position.x += 1,
            _ => (),
        }
    }
}
// ANCHOR_END: run_input_update_moves
