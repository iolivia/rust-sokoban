use ggez::audio;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::time::Duration;

use crate::events::Event;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub struct Renderable {
    paths: Vec<String>,
}

pub enum RenderableKind {
    Static,
    Animated,
}

impl Renderable {
    pub fn new_static(path: &str) -> Self {
        Self {
            paths: vec![path.to_string()],
        }
    }

    pub fn new_animated(paths: Vec<&str>) -> Self {
        Self {
            paths: paths.iter().map(|p| p.to_string()).collect(),
        }
    }

    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn path(&self, path_index: usize) -> String {
        // If we get asked for a path that is larger than the
        // number of paths we actually have, we simply mod the index
        // with the length to get an index that is in range.
        self.paths[path_index % self.paths.len()].clone()
    }
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

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, std::boxed::Box<audio::Source>>,
}
