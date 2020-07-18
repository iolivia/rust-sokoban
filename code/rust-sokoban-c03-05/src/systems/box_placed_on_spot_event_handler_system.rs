use crate::{
    audio::AudioStore,
    events::{BoxPlacedOnSpot, Event},
    resources::EventQueue,
};
use specs::{System, Write};

pub struct EventHandlerSystem {}

// System implementation
impl<'a> System<'a> for EventHandlerSystem {
    // Data
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, AudioStore>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, mut audio_store) = data;

        event_queue.events
            .drain_filter(|x| matches!(x, Event::BoxPlacedOnSpot(_)))
            .for_each(|event| {
                // play sound here
                if let Event::BoxPlacedOnSpot(BoxPlacedOnSpot{ is_correct_spot }) = event {
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };

                audio_store.play_sound(&sound.to_string())

                }

            });
    }
}
