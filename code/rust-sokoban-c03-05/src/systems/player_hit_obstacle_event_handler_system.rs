use crate::{
    audio::AudioStore,
    events::{Event},
    resources::EventQueue,
};
use specs::{System, Write, Read};

pub struct EventHandlerSystem {}

// System implementation
impl<'a> System<'a> for EventHandlerSystem {
    // Data
    type SystemData = (
        Read<'a, EventQueue>,
        Write<'a, AudioStore>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (event_queue, mut audio_store) = data;

        event_queue.events
            .iter()
            .filter(|x| matches!(x, Event::BoxPlacedOnSpot(_)))
            .collect::<Vec<_>>()
            .drain(..)
            .for_each(|event| {
                if let Event::PlayerHitObstacle = event {
                    audio_store.play_sound(&"wall".to_string());
                }
            });
        //event_queue.events
        //    .drain_filter(|x| matches!(x, Event::PlayerHitObstacle) )
        //    .for_each(|event| {
        //        if let Event::PlayerHitObstacle = event {
        //            audio_store.play_sound(&"wall".to_string());
        //        }
        //    });
    }
}
