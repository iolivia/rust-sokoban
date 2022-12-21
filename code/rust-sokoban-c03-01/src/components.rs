use std::fmt::{self, Display};

use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

#[derive(PartialEq)]
pub enum BoxColor {
    Blue,
    Red,
}

impl Display for BoxColor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColor::Red => "red",
            BoxColor::Blue => "blue",
        })?;
        Ok(())
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub color: BoxColor,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub color: BoxColor,
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}
