# First components

We've identified two components so far, Position and Renderable. In this section we are going to implement them!

You might be wondering if we're going to be building our own ECS system or using something off the shelf. The good news is that there are a lot of ECS crates for Rust, so we are quite fortunate to be able to choose one and skip implementing it ourselves. We're going to be using `specs` which is one of the most popular ECS crates out there. So let's add it as a dependency to our project.

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
    z: f32
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String
}
```

You might be wondering what the `#[derive(Component)]` means. This derive macro is saying our struct `Position` implements the `Component` trait. The derive attribute generates code that will implement this trait with its own default implementation on the type Position.

There are a few other useful derive traits that we will be using, but if you want to read more look at [derivable traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) in the rust book.

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

`cannot find derive macro Component in this scope` is pointing us to the fact that we haven't imported `Component` from anywhere, so the compiler doesn't know how to find it. Same for the storage attribute. Both of these are in the specs module so we simply import them.

```rust
// main.rs
...
use specs::{Component, VecStorage};
...
```

If we compile now everything should work. We have a few warnings because we're not using these new components yet but ignore those for now. 

## Creating a world and registering components
Now that we have our components compiling, there is one more thing we need to do. We need to tell `specs` that we are going to use these components in our ECS system. But before we can do that we'll have to create a specs `World`. The world is pretty much what it says on the tin, it will hold all our possible components, entities and systems and is going to nicely manage them for us. 

```rust
// main.rs

...
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

Now we can register our components, let's create a new function to do that

```rust
// main.rs

// This might be the first version we write, but this is actually wrong, 
// because when we pass world as a parameter to this function, we'll either
// take full ownership of it, leaving the caller with nothing, or we will
// get a copy, register the components on the copy, and leave the caller
// with a version of the world that doesn't have the registrations.
pub fn register_components(world: World) {
    world.register::<Position>();
    world.register::<Renderable>();
}

// This version is better, we are passing a reference so we will at least 
// be modifying the right version of the world. The problem is just that 
// though, we have a reference, not a mutable reference so we can't modify 
// anything. If you try to  compile this you'll get an error:
// 
// world.register::<Position>();
//   |     ^^^^^ cannot borrow as mutable
// 
// Rightly so, the compiler is telling us that we need a mutable reference
// of the world in order to make changes to it.
pub fn register_components(world: &World) {
    world.register::<Position>();
    world.register::<Renderable>();
}

// Finally, here is a correct version of the function.
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
}
```

Now let's try to call `register_components` in the main and go through the same process of understanding how we can deal with mutable references.

```rust
// main.rs

// This might be the first version we write. 
// This will give us the following compilation error.
// 
// 53 |     register_components(world);
//   |   ^^^^^
//   |   |
//   |   expected mutable reference, found struct `shred::world::World`
//   |   help: consider mutably borrowing here: `&mut world`
// 
// The compiler even tells us what to do so we do it.
pub fn main() -> GameResult {
    let world = World::new();
    register_components(world);

    ...
    // Create the game state
    let game = &mut Game { world };

    ...
}

// Now we pass &mut world to the function, which means we are borrowing 
// the world, passing that reference to register_components. When
// register_components finishes, we are going to get back the ownership
// to the world. This is almost correct, the only problem now is that 
// we are trying to pass a mutable reference to register_components but we
// don't have one, because the world variable is not declared at mutable.
// the compiler tells us exactly how to fix it, and that's by declaring the
// world as mutable.
//
//    |
//  let world = World::new();
//      ----- help: consider changing this to be mutable: `mut world`
//  register_components(&mut world);
//                      ^^^^^^^^^^ cannot borrow as mutable
pub fn main() -> GameResult {
    let world = World::new();
    register_components(&mut world);

    ...
    // Create the game state
    let game = &mut Game { world };

    ...
}

// Finally, here is the correct version
pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);

    ...
    // Create the game state
    let game = &mut Game { world };

    ...
}

```

This is our final code so you can go ahead and add this to your main and compile.

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

We learnt a few new tricks in this section, so let's recap. We've learnt:
* passing a mutable reference as an argument - `pub fn register_components(world: &mut World)` we are passing the world by mut reference here since the register
* calling a function which requires a mutable reference - `register_components(&mut world)`
* adding a member to a struct - `struct Game { world: World }`
* declaring a variable as mutable when mutable references are required - `let mut world = World::new();`

In this section we've created our first two components, next up we're going to be creating entities!








