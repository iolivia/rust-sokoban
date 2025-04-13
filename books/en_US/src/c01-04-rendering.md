# Rendering system

It's time for our first system, the rendering system. This system will be responsible for drawing all our entities on the screen.

## Rendering system setup

First we'll start with a blank implementation, something like this:

```rust
pub fn run_rendering(world: &World, context: &mut Context) {
        // TODO add implementation
}
```

Finally let's run the rendering system in our draw loop. This means that every time the game updates we will render the latest state of all our entities.

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:handler}}
```

Running the game now should compile, but it will probably not do anything yet, since we haven't filled in any of the implementation of the rendering system and also we haven't created any entities.

## Rendering system implementation

**Note:** We're going to add [glam](https://lib.rs/crates/glam) as a dependency here that is a simple and fast 3D library that offers some performance improvements.

```
{{#include ../../../code/rust-sokoban-c01-04/Cargo.toml:9:11}}
```

Here is the implementation of the rendering system. It does a few things:

* clear the screen (ensuring we don't keep any of the state rendered on the previous frame)
* get all entities with a renderable component and sort them by z (we do this in order to ensure we can render things on top of each other, for example the player should be above the floor, otherwise we wouldn't be able to see them)
* iterate through sorted entities and render each of them as an image
* finally, present to the screen

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:rendering_system}}
```

## Add some test entities

Let's create some test entities to make sure things are working correctly.

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:init}}
```

Finally, let's put everything together and run. You should see something like this! This is super exciting, now we have a proper rendering system and we can actually see something on the screen for the first time. Next up, we're going to work on the gameplay so it can actually feel like a game!

![Screenshot](./images/rendering.png)

Final code below.

> **_NOTE:_**  Note that this is a very basic implementation of rendering and as the number of entities grow the performance will not be good enough. A more advanced implementation of rendering which uses batch rendering can be found in [Chapter 3 - Batch Rendering](/c03-04-batch-rendering.html).

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs}}
```

> **_CODELINK:_**  You can see the full code in this example [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c01-04).
