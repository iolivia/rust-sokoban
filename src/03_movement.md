# Pushing boxes

In the previous chapter we got our player moving, but he is going through walls and boxes, not really interacting with the environment. In this section we'll add some logic for more intelligent player movement.

First, we need to make our code slightly more generic. If you remember the previous chapter we were operating on players to figure out where we should move them, but we'll also need to move boxes. Also in the future we might want to introduce another movable kind of object, so let's try to build something with that in mind. What we'll do in true ECS spirit we will use a marker component to tell us which entities are movable and which aren't. For example, players and boxes are movable, while walls are immovable. Box spots are kind of irrelevant here because they do not move, but they also shouldn't affect the movement of players or boxes, so box spots will not have either of these components. 

Here are our two new components, nothing too new apart from two minor things:
* we are using `NullStorage` which is slightly more efficient than using `VecStorage` since these two components will not have any fields, and are just used as markers
* we are implementing Default because that is a requirement for using NullStorage
* adding the two new compoennts to our register_components function

```rust
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

...
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
```

Next, we'll add:
* with(Movable) to players and boxes
* with(Immovable) to walls
* do nothing with floors and box spots (as mentioned before they should not be part of our movement/collision system since they are inconsequential to the movement)

```rust
// Create a wall entity
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10, ..position})
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .with(Immovable)
        .build();
}

...

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10, ..position})
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Box {})
        .with(Movable)
        .build();
}

...

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10, ..position})
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .with(Movable)
        .build();
}
```

Now let's think of a few examples that illustrate our requirements for movement. This will help us understand how we need to change the implementation of the input system to use `Movable` and `Immovable` correctly.

Scenarios:
1. `(player, floor)` and `RIGHT` pressed -> player should move to the right
1. `(player, wall)` and `RIGHT` pressed -> player should not move to the right
1. `(player, box, floor)` and `RIGHT` pressed -> player should move to the right, box should move to the right
1. `(player, box, wall)` and `RIGHT` pressed -> nothing should move
1. `(player, box, box, floor)` and `RIGHT` pressed -> player, box1 and box2 should all move one to the right
1. `(player, box, box, wall)` and `RIGHT` pressed -> nothing should move

A few observations we can make based on this:
* the collision/movement detection should happen all at once for all objects involved - for example, for scenario 6 if we processed one item at a time, we would move the player, we would move the first box, and when we get to the second box we realize we cannot move it, and we'd have to roll back all our movement actions, which will not work. So for every input, we must figure out all the objects involved and holistically decide if the action is possible or not.
* a chain of movables with an empty spot can move (empty spot in this case means something neither movable or immovable)
* a chain of movables with an immovable spot cannot move
* even though all examples were moving to the right, the rules should generalize for any movement and the key pressed should just influence how we find the chain

So given this, let's start implementing this logic. Let's thing about the logical pieces we need. Some initial ideas:
1. **find all the movable and immovable entities** - this is so we can figure out if they are affected by the movement
2. **figure out which way to move based on a key** - we've kind of figured this out in the previous section already, basically a bunch of +1/-1 operations based on the key enum
3. **iterate through all positions between the player and the end of the map** on the correct axis based on the direction - for example, if we press right, we need to go from player.x to map_width, if we press up we need to go from 0 to player.y
4. **for every tile in this sequence** we need to:
    * if the tile is movable, continue and remember this tile
    * if the tile is not movable, stop and don't move anything 
    * if the tile is neither movable or immovable, move all the tiles we've remembered so far


Great, so let's write down some of this code.

```rust
// get all movable entities into a map (x,y) -> entity_id
let mut mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
    .join()
    .collect::<Vec<_>>()
    .into_iter()
    .map(|t| ((t.2.x, t.2.y), t.0.id()))
    .collect::<HashMap<_, _>>();
```

This piece of code looks a bit intimidating but is quite simple. First we grab all entities which have a movable component and a position component and join them (this should be familiar by now). We then map that Tuple3 into a (x,y) -> entity_id, and finally we convert that to a HashMap so we can index by x and y. As I mentioned previously the fast indexing is crucial so we can quickly check every position in the sequence. 

We then do the same for immovable entities, the code is exactly the same except we join by immovables instead.

```rust
let mut immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
    .join()
    .collect::<Vec<_>>()
    .into_iter()
    .map(|t| ((t.2.x, t.2.y), t.0.id()))
    .collect::<HashMap<_, _>>();
```

Next, we figure out what that sequence of tiles is based on the key that was just pressed.

```rust
let (start, end, is_x) = match key {
    KeyCode::Up => (position.y, 0, false),
    KeyCode::Down => (position.y, MAP_HEIGHT, false),
    KeyCode::Left => (position.x, 0, true),
    KeyCode::Right => (position.x, MAP_WIDTH, true),
    _ => continue
};
```

This code will give us a start and an end and will also tell us if we are about to iterate horizontally (if is_x is true) or vertically. There is a small problem though. Normally we'd do a range `start..=end`, but if `start > end` like it is when we're going left or up this code won't work and the range generated will be empty, so we will not iterate through anything. This is because the range function will check if start <= end and if it's not, it will return an empty iterator. It's not a big deal, but we do need to write some code to handle it. 

```rust
let range = if start < end {
    (start..=end).collect::<Vec<_>>()
} else {
    (end..=start).rev().collect::<Vec<_>>()
};
```

This code will give us a range which is either increasing or decreasing based on the direction we need to go. 

Finally, we iterate through this range and add the logic for movable/immovable we described above. As a reminder, we need to:
    * if the tile is movable, continue and remember this tile
    * if the tile is not movable, stop and don't move anything 
    * if the tile is neither movable or immovable, move all the tiles we've remembered so far

```rust
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
                Some(id) => to_move.clear(),
                None => break
            }
        }
    }
}
```

This will add all the things to be moved into a vector `to_move`. Now all we need to do is process those actions. 

```rust
// Now actually move what needs to be moved
for (key, id) in to_move {
    let position = positions.get_mut(entities.entity(id));
    if let Some(position) = position {
        match key {
            KeyCode::Up => position.y -= 1,
            KeyCode::Down => position.y += 1,
            KeyCode::Left => position.x -= 1,
            KeyCode::Right => position.x += 1,
            _ => ()
        }
    }
}
```

And here is the code in action.

![Sokoban moving boxes](./images/moving_boxes.gif)

Final code below.

```rust
use specs::Entities;
use specs::NullStorage;
use specs::WriteStorage;
use ggez;
use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::nalgebra as na;
use ggez::{conf, event, Context, GameResult};
use specs::{
    join::Join, Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt, Write,
};
use specs::world::Index;

use std::path;
use std::collections::HashMap;

const TILE_WIDTH: f32 = 32.0;
const MAP_WIDTH: u8 = 10;
const MAP_HEIGHT: u8 = 10;
const MAP_OFFSET_X: u8 = 4;
const MAP_OFFSET_Y: u8 = 3;

// Components
#[derive(Debug, Component, Clone, Copy)]
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

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>
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
            let x = (MAP_OFFSET_X + position.x) as f32 * TILE_WIDTH;
            let y = (MAP_OFFSET_Y + position.y) as f32 * TILE_WIDTH;
            
            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}

pub struct InputSystem {}

// System implementation
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

        let (
            mut input_queue, 
            entities, 
            mut positions, 
            players, 
            movables, 
            immovables
        ) = data;
        
        let mut to_move = Vec::new();
        
        for (position, _player) in (&positions, &players).join() {
            // Get the first key pressed
            if let Some(key) = input_queue.keys_pressed.pop() {

                // get all the movables and immovables
                let mut mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let mut immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                // Now iterate through current position to the end of the map
                // on the correct axis and check what needs to move.
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue
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
                                Some(id) => to_move.clear(),
                                None => break
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
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => ()
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
impl event::EventHandler for Game {
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
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool
    ) {
        println!("Key pressed: {:?}", keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
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
    world.register::<Movable>();
    world.register::<Immovable>();
}

// Register resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}

// Z positions
// Wall: 10
// Floor: 5
// Box: 10
// Box spot: 9
// Player: 10

// Create a wall entity
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {z: 10, ..position})
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
        .with(Movable)
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
        .with(Movable)
        .build();
}

pub fn create_map(world: &mut World) {
    let no_op = |_world: &mut World, _position: Position| {};

    for x in 0..=MAP_WIDTH {
        for y in 0..=MAP_HEIGHT {

            // Create the position at which to create something on the map
            let position = Position {
                x: x,
                y: y,
                z: 0 // we will get the z from the factory functions
            };

            // Figure out what object we should create
            let create = match (x, y) {
                (x, y) if x == 0 || x == MAP_WIDTH || y == 0 || y == MAP_HEIGHT => create_wall,
                (5, 5) => create_player,
                (6, 5) => create_box,
                (7, 5) => create_box,
                (8, 2) => create_box_spot,
                _ => no_op,
            };

            // Create floor and create the special objects
            create_floor(world, position);
            create(world, position);
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
    register_resources(&mut world);
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







