# Map

It's time to create our first basic map. The basic idea is:
* we'll start with a 10x10 grid of tiles
* we'll put walls on the borders
* we'll put the player at 5,5
* we'll put one box at 7,7
* we'll put the box spot at 8,2
* floors everywhere else

Here is the code to generate this basic map.

```rust
pub fn create_map(world: &mut World) {
    let width = 10;
    let height = 10;

    for x in 0..=width {
        for y in 0..=height {
            // create walls on the borders
            if x == 0 || x == width || y == 0 || y == height {
                create_wall(
                    world,
                    Position {
                        x: TILE_WIDTH * x as f32,
                        y: TILE_WIDTH * y as f32,
                    },
                );
            } else {
                // check if it's any other type of entity
                let create = match (x, y) {
                    (5, 5) => create_player,
                    (7, 7) => create_box,
                    (8, 2) => create_box_spot,
                    _ => create_floor,
                };
                create(
                    world,
                    Position {
                        x: TILE_WIDTH * x as f32,
                        y: TILE_WIDTH * y as f32,
                    },
                );
            }
        }
    }
}

// Initialize the level - replace the dummy implementation
// of initialize level we had before
pub fn initialize_level(world: &mut World) {
    create_map(world);
}
```

There are a few new things here:
* for loops with ranges - ` for x in 0..=width` this is much like a regular for loop where we go from 0 to width (the `=` means it's inclusive)
* matches - matches are like if else statements on steroids. with a match we can do much more complex pattern match, in this case we are matching the x and y values and we return a different function reference. 
 
If we run this code we should now see a pretty map.

![Screenshot of map](./images/window_map.png)

One final touch, let's make the map a bit more centered. I took this opportunity to also refactor and simplify `create_map`. Now handing the borders is also done part of the match so we were able to simplify the code. Notice how we are now using a fancier match where we match on `(x, y)` with a condition: `if x == 0 || x == width || y == 0 || y == height`.


```rust
pub fn create_map(world: &mut World) {
    let width = 10;
    let height = 10;
    let (offset_x, offset_y) = (4, 3); // make the map somewhat centered

    for x in 0..=width {
        for y in 0..=height {
            let create = match (x, y) {
                (x, y) if x == 0 || x == width || y == 0 || y == height => create_wall,
                (5, 5) => create_player,
                (7, 7) => create_box,
                (8, 2) => create_box_spot,
                _ => create_floor,
            };
            create(
                world,
                Position {
                    x: TILE_WIDTH * (x + offset_x) as f32,
                    y: TILE_WIDTH * (y + offset_y) as f32,
                    z: 0.0 // we will get the z from the factory functions
                },
            );
        }
    }
}
```

![Screenshot of map](./images/window_map_centered.png)

Next up we'll spend some time making our code nicer, moving things around and starting on the input system so we can move our player around. Come along for the ride!

Full code below. 

```rust
use ggez;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::nalgebra as na;
use ggez::{conf, event, Context, GameResult};
use specs::{
    join::Join, Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
};

use std::path;

const TILE_WIDTH: f32 = 32.0;

// Components
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
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

        // Clearing the screen (this gives us the backround colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&a, &b| a.0.z.partial_cmp(&b.0.z).expect("expected comparison"));

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");

            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(position.x, position.y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
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
impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
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
}

// Register components with the world
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
}

// Create a wall entity
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10.0, ..position})
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 5.0, ..position})
        .with(Renderable {
            path: "/images/floor.png".to_string(),
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10.0, ..position})
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Box {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 9.0, ..position})
        .with(Renderable {
            path: "/images/box_spot.png".to_string(),
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10.0, ..position})
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .build();
}

pub fn create_map(world: &mut World) {
    let width = 10;
    let height = 10;
    let (offset_x, offset_y) = (4, 3); // make the map somewhat centered

    for x in 0..=width {
        for y in 0..=height {
            let create = match (x, y) {
                (x, y) if x == 0 || x == width || y == 0 || y == height => create_wall,
                (5, 5) => create_player,
                (7, 7) => create_box,
                (8, 2) => create_box_spot,
                _ => create_floor,
            };
            create(
                world,
                Position {
                    x: TILE_WIDTH * (x + offset_x) as f32,
                    y: TILE_WIDTH * (y + offset_y) as f32,
                },
            );
        }
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    create_map(world);
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
    let game = &mut Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
```
