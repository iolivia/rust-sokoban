use crate::audio::AudioStore;
use crate::events::Event;
use ggez::event::KeyCode;
use specs::World;
use std::fmt;
use std::{fmt::Display, time::Duration};

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

// ANCHOR: register_resources_1
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    // ANCHOR_END: register_resources_1
    // ANCHOR: register_resources_2
    world.insert(AudioStore::default());
    // ANCHOR_END: register_resources_2
    // ANCHOR: register_resources_end
}
// ANCHOR_END: register_resources_end

pub enum GameplayState {
    Playing,
    Won,
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

// ANCHOR: event_queue
#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}
// ANCHOR_END: event_queue
