// ANCHOR: include_start
use crate::{
    // ANCHOR_END: include_start
    // ANCHOR: include_audio_store
    audio::AudioStore,
    // ANCHOR_END: include_audio_store
    // ANCHOR: include_end
    components::*,
    events::{BoxPlacedOnSpot, EntityMoved, Event},
    resources::EventQueue,
};
use specs::{Entities, Join, ReadStorage, System, Write};
use std::collections::HashMap;
// ANCHOR_END: include_end

pub struct EventSystem {}

// System implementation
// ANCHOR: event_sys_impl_start
impl<'a> System<'a> for EventSystem {
    // ANCHOR_END: event_sys_impl_start
    // ANCHOR: sys_data_1
    // Data
    type SystemData = (
        Write<'a, EventQueue>,
    // ANCHOR_END: sys_data_1
    // ANCHOR: sys_data_audio
        Write<'a, AudioStore>,
    // ANCHOR_END: sys_data_audio
    // ANCHOR: sys_data_2
        Entities<'a>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );
    // ANCHOR_END: sys_data_2

    // ANCHOR: run_start
    // ANCHOR: run_full
    fn run(&mut self, data: Self::SystemData) {
        // ANCHOR_END: run_start
        // ANCHOR: run_let_data
        let (mut event_queue, mut audio_store, entities, boxes, box_spots, positions) = data;
        // ANCHOR_END: run_let_data

        // ANCHOR: run_body_1
        let mut new_events = Vec::new();

        for event in event_queue.events.drain(..) {
            println!("New event: {:?}", event);

            // ANCHOR: play_sound_1
            match event {
                Event::PlayerHitObstacle => {
                    // play sound here
                    // ANCHOR_END: run_body_1
                    audio_store.play_sound(&"wall".to_string());
                    // ANCHOR: run_body_2
                }
                Event::EntityMoved(EntityMoved { id }) => {
                    // ANCHOR_END: play_sound_1
                    // An entity was just moved, check if it was a box and fire
                    // more events if it's been moved on a spot.
                    if let Some(the_box) = boxes.get(entities.entity(id)) {
                        let box_spots_with_positions: HashMap<(u8, u8), &BoxSpot> =
                            (&box_spots, &positions)
                                .join()
                                .map(|t| ((t.1.x, t.1.y), t.0))
                                .collect::<HashMap<_, _>>();

                        if let Some(box_position) = positions.get(entities.entity(id)) {
                            // Check if there is a spot on this position, and if there
                            // is if it's the correct or incorrect type
                            if let Some(box_spot) =
                                box_spots_with_positions.get(&(box_position.x, box_position.y))
                            {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: (box_spot.colour == the_box.colour),
                                }));
                            }
                        }
                    }
                    // ANCHOR: play_sound_2
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    // play sound here
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };

                    audio_store.play_sound(&sound.to_string())
                    // ANCHOR: run_body_3
                }
            }
            // ANCHOR_END: play_sound_2
        }

        event_queue.events.append(&mut new_events);
        // ANCHOR: run_end
    }
}
// ANCHOR_END: run_end
// ANCHOR_END: run_body_3
// ANCHOR_END: run_full
