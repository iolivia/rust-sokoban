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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
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
