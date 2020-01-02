# First components

We've identified two components so far, Position and Renderable. In this section we are going to implement them!

You might be wondering if we're going to be building our own ECS system or using something off the shelf. The good news is that there are a lot of ECS crates for Rust, so we are quite fortunate to be able to choose one and skip implementing it ourselves. We're going to be using `specs` which is one of the most popular ECS crates out there. So let's add it as a dependency to our project. We're actually adding two crates - `specs` and `specs-derive`.

```toml
[dependencies]
ggez = "0.5.1"
specs = { version = "0.15.0", features = ["specs-derive"] }
```

## Defining the Position and Renderable components
Now let's define our first two components. The position component holds the x and y components we already discussed and the renderable component holds the path to an asset. We will create different assets for walls, floors, etc. and we'll pass in those paths when we create the entities and components associated with those entities.

```rust
#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String
}
```

## Making the compiler our friend 
If you try to compile this code with `cargo build` you will notice we get some errors. Using the compiler to help you is a big part of using Rust effectively, so let's try to work with the compiler and understand the errors.

```rust 
error: cannot find derive macro `Component` in this scope
  --> src/main.rs:16:10
   |
16 | #[derive(Component)]
   |          ^^^^^^^^^

error: cannot find attribute `storage` in this scope
  --> src/main.rs:17:3
   |
17 | #[storage(VecStorage)]
   |   ^^^^^^^
```

`cannot find derive macro Component in this scope` is pointing us to the fact that we haven't imported `Component` from anywhere, so the compiler doesn't know how to find it. We need to do two things:
* import `Component` 
* enable macro usage from `specs_derive`

```rust
// Enable macro usage
#[macro_use]
extern crate specs_derive;

// Import component
use specs::Component;
```

TODO clarify >
You might be wondering what the `#[derive(Component)]` means. This derive macro is saying our struct `Position` implements a `Component` trait. The derive attribute generates code that will implement a trait with its own default implementation on the type you’ve annotated with the derive syntax. There are a few other useful derive traits that we will be using, but if you want to read more look at [derivable traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) in the rust book.

Now if we try to compile again we see the component error is gone but we still have one about storage. This time the compiler is even suggesting what we should import, so let's listen to it and `use specs::VecStorage;`

```rust
error[E0412]: cannot find type `VecStorage` in this scope
  --> src/main.rs:11:11
   |
11 | #[storage(VecStorage)]
   |           ^^^^^^^^^^ not found in this scope
   |
help: possible candidates are found in other modules, you can import them into scope
   |
5  | use specs::VecStorage;
   |
5  | use specs::prelude::VecStorage;
   |
5  | use specs::storage::VecStorage;
```

If we compile now everything should work. We have a few warnings because we're not using these new components yet but ignore those for now. 

```rust
   Compiling rust-sokoban v0.1.0 (/Users/olivia/repos/rust-game-book/code/rust-sokoban-02)

   ... warnings ..

    Finished dev [unoptimized + debuginfo] target(s) in 5.74s
```

## Creating a world and registering components
Now that we have our components compiling, there is one more thing we need to do. We need to tell `specs` that we are going to use these components in our ECS system. But before we can do that we'll have to create a specs `World`. The world is pretty much what it says on the tin, it will hold all our possible components, entities and systems and is going to nicely manage them for us. 

```rust
// main.rs

// Import world
use specs::{World, WorldExt};

// Add a world member to our Game struct
struct Game {
    world: World
}

...

// Create a new world and pass it to our Game struct
pub fn main() -> GameResult {
    ...

    // Create the game state
    let game = &mut Game {
        world: World::new()
    };
    ...
}
```

Now we can register our components, let's create a new function to do that.

```rust
// main.rs

...

// Register components with the world
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
}

// Call register_components in main
pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);

    ...
    // Create the game state
    let game = &mut Game { world };

    ...
}

```








