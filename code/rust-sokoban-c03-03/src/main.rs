/* ANCHOR: all */
// Rust sokoban
// main.rs

use ggez::{
    conf, event,
    graphics::{self, DrawParam, Image},
    input::keyboard::{self, KeyCode},
    timer, Context, GameResult,
};
use glam::Vec2;
use hecs::{Entity, World};

use std::collections::HashMap;
use std::path;

mod components;
mod constants;
mod entities;
mod events;
mod map;
mod systems;

// ANCHOR: game
// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
}
// ANCHOR_END: game

// ANCHOR: handler
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // Run input system
        {
            systems::input::run_input(&self.world, context);
        }

        // Run gameplay state
        {
            systems::gameplay::run_gameplay_state(&mut self.world);
        }

        // Run events processing
        {
            systems::events::run_process_events(&mut self.world);
        }

        // Get and update time resource
        {
            let mut query = self.world.query::<&mut crate::components::Time>();
            let mut time = query.iter().next().unwrap().1;
            time.delta += timer::delta(context);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            systems::rendering::run_rendering(&self.world, context);
        }

        Ok(())
    }
}
// ANCHOR_END: handler

// ANCHOR: main
pub fn main() -> GameResult {
    let mut world = World::new();
    map::initialize_level(&mut world);
    entities::create_gameplay(&mut world);
    entities::create_time(&mut world);
    entities::create_event_queue(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
// ANCHOR_END: main

/* ANCHOR_END: all */
