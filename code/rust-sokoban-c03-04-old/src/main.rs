// Rust sokoban
// main.rs
use ggez::{conf, event::{self, KeyCode, KeyMods}, timer, Context, GameResult};
use specs::{RunNow, World, WorldExt};

use std::path;

mod audio;
mod components;
mod constants;
mod entities;
mod events;
mod map;
mod resources;
mod systems;

use crate::audio::*;
use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

struct Game {
    world: World,
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        // Get and update time resource
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(context);
        }

        // Run event system
        {
            let mut es = EventSystem { context };
            es.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;
    initialize_sounds(&mut world, &mut context);

    // Create the game state
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
