use std::fmt;
use std::fmt::Display;
use std::time::Duration;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

// ANCHOR: renderable
pub struct Renderable {
    paths: Vec<String>,
}
// ANCHOR_END: renderable

// ANCHOR: renderable_kind
pub enum RenderableKind {
    Static,
    Animated,
}
// ANCHOR_END: renderable_kind

impl Renderable {
    // ANCHOR: renderable_new_fn
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
    // ANCHOR_END: renderable_new_fn

    // ANCHOR: renderable_kind_fn
    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }
    // ANCHOR_END: renderable_kind_fn

    // ANCHOR: renderable_path_fn
    pub fn path(&self, path_index: usize) -> String {
        // If we get asked for a path that is larger than the
        // number of paths we actually have, we simply mod the index
        // with the length to get an index that is in range.
        self.paths[path_index % self.paths.len()].clone()
    }
    // ANCHOR_END: renderable_path_fn
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

#[derive(Default)]
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

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}
