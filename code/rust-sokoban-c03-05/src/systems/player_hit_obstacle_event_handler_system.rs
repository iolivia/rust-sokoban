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
        Write<'a, EventQueue>,
        Write<'a, AudioStore>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let ( mut event_queue, mut audio_store) = data;

        let mut new_events = Vec::new();
        event_queue.events
            .drain(..)
            .for_each(|event| {
                println!("player hit {:?}", event);
                if let Event::PlayerHitObstacle = event {
                    audio_store.play_sound(&"wall".to_string());
                } else {
                    new_events.push(event);
                }
            });
        event_queue.events.append(&mut new_events);
        //event_queue.events
        //    .drain_filter(|x| matches!(x, Event::PlayerHitObstacle) )
        //    .for_each(|event| {
        //        if let Event::PlayerHitObstacle = event {
        //            audio_store.play_sound(&"wall".to_string());
        //        }
        //    });
    }
}
