# First entities

So now we have our two components `Position` and `Renderable`, but we need to create entities. For this we're going to need our first assets. Feel free to create your own here or just use the assets in this book!

Floor tile

![Floor tile](./images/floor.png)

Wall tile

![Wall tile](./images/wall.png)

First let's add the images to our project. We'll add a `resources` folder which will hold the images and any other configuration or assets we might have. It should look like this.

```
- Cargo.toml
- .gitignore
- src
- resources
-- images
--- floor.png
--- wall.png
-- main.rs
```

Now let's write the function to create the floor entity.

```rust
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
```

We're taking the world, creating an entity with `create_entity` and then we are adding components using `with`. We're also passing in the path to the wall asset in the resources/images folder. At the end we're calling build which will give us the entity.

Finally, we'll write a function to initialize the level, create one wall entity at (0, 0) and add the resources folder to our ggez initialization. Here is the full code.

```rust
// main.rs

use ggez;
use ggez::{conf, event, Context, GameResult};
use specs::{Builder, World, WorldExt};
use specs::{Component, VecStorage};

use std::path;

// Components
#[derive(Component)]
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
        // TODO: update game logic here
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update draw here
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
    create_wall(world, Position { x: 0.0, y: 0.0 });
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

If you run this, you'll notice the wall doesn't actually appear on the screen. But there are no errors either. What's happenning is that we haven't done anything to render yet. In the next section we'll be writing our first system, the rendering

