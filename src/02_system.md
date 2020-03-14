# First system

The first system we are going to create is the rendering system. This will help us draw everything we have set up so far on the screen. 

Let's go through the system implementation step by step. First, we create a Rendering system struct. The only thing this struct needs is the ggez context. We'll need the context in order to pretty much do anything graphics related.

```rust
// Systems
pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}
```

At this point you might be wondering what is that weird `<'a>` syntax. And all I can really tell you at this point is that those are [lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html). You'll need them when you use references and it's basically going to help the compiler figure out if at any point we have any references that might not be valid. In this case we have the `'a` annotation on our struct `RenderingSystem` but also on the `context` member variable. All we are saying is that an instance of `RenderingSystem` and that specific instance of `context` both live for the duration of the `'a` lifetime, so it means we'll never get in trouble (i.e. we will never have a valid system but not context or the other way around). The syntax of lifetimes can be a bit daunting, but you'll get used to it.

Let's move on to implementing the rendering system. We'll have to import the `System` trait from specs, I've also taken this opportunity to collapse all the specs imports into one line, so here it is.

```rust
use specs::{
    join::Join, Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
};
```

Now let's make `RenderingSystem` implement `System`. This should be very similar to what we have learnt in the previous section where we implemented the `Event` trait to set up our game loop. If we take a peek at the `System` trait we'll see we have to implement this run function that looks like this `fn run(&mut self, data: Self::SystemData);` and the `SystemData` type.

```rust
// This is our first attempt but if we compile this we'll get a bunch of scary lifetime errors.
// error[E0726]: implicit elided lifetime not allowed here
//    |
// 34 | impl System for RenderingSystem {
//    |      ^^^^^^- help: indicate the anonymous lifetime: `<'_>`
//
// error[E0726]: implicit elided lifetime not allowed here
//   --> src/main.rs:34:17
//    |
// 34 | impl System for RenderingSystem {
//    |                 ^^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
impl System for RenderingSystem {
    // empty type for now
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
    }
}
```

All those errors are complaining about lifetimes, and if we look again at the `System` trait we'll see it actually has a lifetime parameter `pub trait System<'a>`. So let's add the lifetime everywhere. 

```rust
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
    }
}
```

Everything should be compiling now. See, lifetimes are not that scary. Now we'll implement the code for iterating through tuples of components so we can eventually render them.

```rust
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    // Running
    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in (&positions, &renderables).join() {
        }
    }
}
```

We are doing a few things here:
* accessing `ReadStorage<'a, Position>` and `ReadStorage<'a, Renderable>` in the system. `SystemData` is a struct of storages that define what this system has access to. For now, this system needs the positions and renderable components, and because we are using `Read` we are saying we only require read access (rendering should not modify the components, simply present them on the screen)
* destructuring a tuple: `let (positions, renderables) = data;`, you might be familiar with this syntax from other languages but if not, this is unpacking the individual elements of `data` and giving them custom names. If we didn't do this we could access the components of data like this:
** `data.0` for position storage
** `data.1` for renderable storage
The index syntax works but I find it less readable than the destructuring, especially when we'll have more than 2 component storages.
* looping through positions and renderables together using `join`: the join syntax is a handy `specs` trick that allows us to iterate through all the entities that have both positions and renderable components. let's take an example to see how that might work:
// scenario 1: 
// entity 1: pos, ren
// entity 2: pos
// entity 3: pos, ren
// entity 4: ren
// (&positions, &renderables).join() -> will contain entity 1 and entity 3, because they are the only two entities that have both pos and ren.

You can read more about [joins](https://specs.amethyst.rs/docs/tutorials/08_join.html) in the specs book. 

Finally, let's implement the graphics bits: loading images and drawing them. Here is the code for the system.


```rust
// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        // Clearing the screen (this gives us the backround colour)
        graphics::clear(self.context, graphics::Color::new(1.0, 1.0, 1.0, 1.0));

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in (&positions, &renderables).join() {
            // Load the image
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            
            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}
```

There are a few new Rust tips in here:
* `.clone()` - notice when we create a new image we pass the context and `renderable.path.clone()`. This simply means we make a copy of `renderable.path` and that is what we pass to the function. If we remove the clone the error we get is `   |                                                  ^^^^^^^^^^^^^^^ move occurs because renderable.path has type std::string::String, which does not implement the Copy trait`. This means the compiler will never attempt to copy a string, the copying will always have to be explicit and this is why we call `clone` ourselves
* `.expect` - notice we call `.expect("expected image");` after we try to load the image. That is because the `new` function returns a `GameResult<Image>`, which underneath is simply `Result<Image, GameError>;`. Results are one of the nicest things about Rust and it gives us a nice way of dealing with failure and success. For example, in this case this signature means that when we call `new` we will either get an `Image` if everything goes well, but if it doesn't we'll get a `GameError`. This is great because we don't need to deal with exceptions, you can read more about results in the [Error handling chapter](https://doc.rust-lang.org/book/ch09-00-error-handling.html) of the rust book.

This system will get all components and render each of them. There is only one issue with this implementation, and it's that it will not respect the z position of the entities. This means that we will render in the order in which entities get added to our storage rather than the intentional order we want to layer floors with players and boxes and walls. So let's go ahead and sort our entities by the z positions. We'll still use join for the intial step, but we'll add the sorting after that before we iterate. Here's how that code looks like. 

```rust
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
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            
            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}
```

Finally, let's run the code and we should see our first wall being rendered. Full code below.

![Screenshot window with wall](./images/window_wall.png)

```rust
use ggez;
use ggez::event::KeyCode;
use ggez::event::KeyMods;
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
const MAP_WIDTH: u8 = 10;
const MAP_HEIGHT: u8 = 10;

// Components
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8
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
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            
            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
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

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
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
        .with(Position {z: 10, ..position})
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 5, ..position})
        .with(Renderable {
            path: "/images/floor.png".to_string(),
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10, ..position})
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Box {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 9, ..position})
        .with(Renderable {
            path: "/images/box_spot.png".to_string(),
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10, ..position})
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .build();
}

pub fn create_map(world: &mut World) {
    let (offset_x, offset_y) = (4, 3); // make the map somewhat centered

    for x in 0..=MAP_WIDTH {
        for y in 0..=MAP_HEIGHT {
            let create = match (x, y) {
                (x, y) if x == 0 || x == MAP_WIDTH || y == 0 || y == MAP_HEIGHT => create_wall,

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
                    z: 0 // we will get the z from the factory functions
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