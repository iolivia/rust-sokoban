// Rust sokoban
// main.rs

use ggez::{
    conf, event,
    graphics::{self, Canvas, DrawParam, Image},
    input::keyboard::KeyInput,
    winit::event::VirtualKeyCode,
    Context, GameResult,
};
use glam::Vec2;
use specs::{
    join::Join, world::Index, Builder, Component, Entities, NullStorage, ReadStorage, RunNow,
    System, VecStorage, World, WorldExt, Write, WriteStorage,
};
use std::{collections::HashMap, path};

const TILE_WIDTH: f32 = 32.0;
const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT: u8 = 9;

// Components
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<VirtualKeyCode>,
}

// Systems
pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        // Clearing the screen (this gives us the background colour)
        let mut canvas =
            Canvas::from_frame(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image =
                Image::from_path(self.context, renderable.path.clone()).expect("expected image");
            let x: f32 = position.x as f32 * TILE_WIDTH;
            let y: f32 = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            canvas.draw(&image, draw_params);
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        canvas.finish(self.context).expect("expected to present");
    }
}

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Data
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, movables, immovables) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            // Get the first key pressed
            if let Some(key) = input_queue.keys_pressed.pop() {
                // get all the movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                // Now iterate through current position to the end of the map
                // on the correct axis and check what needs to move.
                let (start, end, is_x) = match key {
                    VirtualKeyCode::Up => (position.y, 0, false),
                    VirtualKeyCode::Down => (position.y, MAP_HEIGHT, false),
                    VirtualKeyCode::Left => (position.x, 0, true),
                    VirtualKeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // find a movable
                    // if it exists, we try to move it and continue
                    // if it doesn't exist, we continue and try to find an immovable instead
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => {
                            // find an immovable
                            // if it exists, we need to stop and not move anything
                            // if it doesn't exist, we stop because we found a gap
                            match immov.get(&pos) {
                                Some(_id) => to_move.clear(),
                                None => break,
                            }
                        }
                    }
                }
            }
        }

        // Now actually move what needs to be moved
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    VirtualKeyCode::Up => position.y -= 1,
                    VirtualKeyCode::Down => position.y += 1,
                    VirtualKeyCode::Left => position.x -= 1,
                    VirtualKeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
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
        keyinput: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keyinput.keycode.unwrap());
        Ok(())
    }
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}

// Register components with the world
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Immovable>();
    world.register::<Movable>();
}

// Create a wall entity
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

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Box {})
        .with(Movable)
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable {
            path: "/images/box_spot.png".to_string(),
        })
        .with(BoxSpot {})
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

pub fn load_map(world: &mut World, map_string: String) {
    // read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W 
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
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

    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
