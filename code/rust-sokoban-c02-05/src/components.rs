use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub struct Renderable {
    pub path: String,
}

pub struct Wall {}

pub struct Player {}

pub struct Box {}

pub struct BoxSpot {}

pub struct Movable;

pub struct Immovable;

// ANCHOR: gameplay_state
#[derive(Default)]
pub enum GameplayState {
    #[default]
    Playing,
    Won,
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}
// ANCHOR_END: gameplay_state

// ANCHOR: gameplay_state_impl_default
// ANCHOR_END: gameplay_state_impl_default

// ANCHOR: gameplay_state_impl_display
impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}
// ANCHOR_END: gameplay_state_impl_display
