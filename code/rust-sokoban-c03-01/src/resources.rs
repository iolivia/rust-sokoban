use ggez::winit::event::VirtualKeyCode;
use specs::World;
use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<VirtualKeyCode>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum GameplayState {
    #[default]
    Playing,
    Won,
}

impl Display for GameplayState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            GameplayState::Playing => write!(f, "Playing"),
            GameplayState::Won => write!(f, "Won"),
        }
    }
}
#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
}
