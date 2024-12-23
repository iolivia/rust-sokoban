use crate::components::*;
use crate::events::*;
use hecs::World;

use std::collections::HashMap;

pub fn run_process_events(world: &mut World) {
    let events = {
        let mut query = world.query::<&mut crate::components::EventQueue>();
        let events = query
            .iter()
            .next()
            .unwrap()
            .1
            .events
            .drain(..)
            .collect::<Vec<_>>();

        events
    };

    let mut new_events = Vec::new();

    let mut query = world.query::<(&Position, &BoxSpot)>();
    let box_spots_by_position: HashMap<(u8, u8), &BoxSpot> = query
        .iter()
        .map(|(_, t)| ((t.0.x, t.0.y), t.1))
        .collect::<HashMap<_, _>>();

    for event in events {
        println!("New event: {:?}", event);

        match event {
            Event::PlayerHitObstacle => {
                // play sound here
            }
            Event::EntityMoved(EntityMoved { entity }) => {
                // An entity was just moved, check if it was a box and fire
                // more events if it's been moved on a spot.
                if let Ok(the_box) = world.get::<&Box>(entity) {
                    if let Ok(box_position) = world.get::<&Position>(entity) {
                        // Check if there is a spot on this position, and if there
                        // is if it's the correct or incorrect type
                        if let Some(box_spot) =
                            box_spots_by_position.get(&(box_position.x, box_position.y))
                        {
                            new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                is_correct_spot: (box_spot.colour == the_box.colour),
                            }));
                        }
                    }
                }
            }
            Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot: _ }) => {
                // play sound here
            }
        }
    }

    // Finally add events back into the world
    {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut new_events);
    }
}
