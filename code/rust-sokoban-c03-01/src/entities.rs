use crate::components::*;
use specs::{Builder, World, WorldExt};

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .with(Immovable)
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: "/images/floor.png".to_string(),
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position, color: BoxColor) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: format!("/images/box_{}.png", color),
        })
        .with(Box { color })
        .with(Movable)
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position, color: BoxColor) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable {
            path: format!("/images/box_spot_{}.png", color),
        })
        .with(BoxSpot { color })
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .with(Movable)
        .build();
}
