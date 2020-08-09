use ggez::event::KeyCode;
use specs::World;
use std::fmt;
use std::fmt::Display;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

// ANCHOR: register_resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
}
// ANCHOR_END: register_resources

// ANCHOR: gameplay_state
pub enum GameplayState {
    Playing,
    Won
}
// ANCHOR_END: gameplay_state

// ANCHOR: gameplay_state_display
impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won"
        })?;
        Ok(())
    }
}
// ANCHOR_END: gameplay_state_display

// ANCHOR: gameplay_state_default
impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}
// ANCHOR_END: gameplay_state_default

// ANCHOR: gameplay
#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}
// ANCHOR_END: gameplay
