use ggez;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::nalgebra as na;
use ggez::{conf, event, Context, GameResult};
use specs::join::Join;
use specs::ReadStorage;
use specs::System;
use specs::{Builder, World, WorldExt};
use specs::{Component, RunNow, VecStorage};

use std::path;

// Components
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String,
}

// Systems
pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        graphics::clear(self.context, graphics::Color::new(0.71, 0.9, 0.51, 1.0));

        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        let r2 = graphics::Mesh::new_rectangle(
            self.context,
            graphics::DrawMode::stroke(1.0),
            rect,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )
        .expect("e");
        graphics::draw(self.context, &r2, DrawParam::default()).expect("f");
        for (position, renderable) in (&positions, &renderables).join() {
            println!("here {:#?}", position);
            // Load the image
            let image = Image::new(self.context, renderable.path.clone())
                .expect("expected game spritesheet");

            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(position.x, position.y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }
    }
}

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
    pos_x: f32,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // TODO: update game logic here

        self.pos_x = self.pos_x % 800.0 + 1.0;

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        // {
        //     let mut rs = RenderingSystem { context };
        //     rs.run_now(&self.world);
        // }

        // let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        // let r2 = graphics::Mesh::new_rectangle(
        //     context,
        //     graphics::DrawMode::stroke(1.0),
        //     rect,
        //     graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        // )
        // .expect("e");
        // graphics::draw(context, &r2, DrawParam::default()).expect("f");

        graphics::clear(context, [0.1, 0.2, 0.3, 1.0].into());
        let circle = graphics::Mesh::new_circle(
            context,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(context, &circle, (na::Point2::new(100.0, 380.0),))?;

        Ok(())
    }
}

// Register components with the world
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
}

// Create a wall entity
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(position)
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .build();
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    create_wall(world, Position { x: 10.0, y: 10.0 });
    // create_wall(world, Position { x: 100.0, y: 100.0 });
    // create_wall(world, Position { x: 1.0, y: 100.0 });
    // create_wall(world, Position { x: 440.0, y: 140.0 });
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    initialize_level(&mut world);
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));
    let (context, event_loop) = &mut context_builder.build()?;
    // Create the game state
    let game = &mut Game { world, pos_x: 0.0 };
    // Run the main event loop
    event::run(context, event_loop, game)
}
