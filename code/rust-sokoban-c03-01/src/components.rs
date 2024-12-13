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

#[derive(PartialEq)]
pub enum BoxColour {
    Red,
    Blue,
}

impl Display for BoxColour {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        })?;
        Ok(())
    }
}

pub struct Box {
    pub colour: BoxColour,
}

pub struct BoxSpot {
    pub colour: BoxColour,
}

pub struct Movable;

pub struct Immovable;

pub enum GameplayState {
    Playing,
    Won,
}

impl Default for GameplayState {
    fn default() -> Self {
        GameplayState::Playing
    }
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
