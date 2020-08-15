# Modules

The main file is getting quite big and as you can imagine, that will not be very sustainable as our project grows. Luckily, Rust has the concept of modules which will alow us to nicely split out functionality based on concerns into separate files.

For now, let's aim for this folder structure. Eventually as we get more components and systems, we'll probably want more than one file, but this should be a pretty good place to start.

```
├── resources
│   └── images
│       ├── box.png
│       ├── box_spot.png
│       ├── floor.png
│       ├── player.png
│       └── wall.png
├── src
│   ├── systems
│   │   ├── input_system.rs
│   │   ├── mod.rs
│   │   └── rendering_system.rs
│   ├── components.rs
│   ├── constants.rs
│   ├── entities.rs
│   ├── main.rs
│   ├── map.rs
│   └── resources.rs
└── Cargo.toml
```

> **_MORE:_**  Read more about modules and managing growing projects [here](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html).

Let's start by moving all the components into a file. There should be no changes apart from making some fields public. The reason why we need to make the fields public is because when everything was in the same file everything had access to everything else, which was convenient to start with, but now that we have split things out we need to pay more attention to visibilities. For now we'll make the fields public to get things working again, but there is a better way which we will discuss in a later section. We've also moved the components registration at the bottom of this file which is quite handy when we add components we only need to change this file.

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c02-04/src/components.rs:}}
```

Now for the resources.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c02-04/src/resources.rs:}}
```

Next up, let's move the constants into their own file. For now we are hardcoding the map dimensions, we need them for the movement to know when we've reached the edge of the map, but as an improvement would could later store the dimensions of the map and make them dynamic based on the map loading.

```rust
// constants.rs
{{#include ../../../code/rust-sokoban-c02-04/src/constants.rs}}
```

Next up, entity creation code is now into an entities file.

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c02-04/src/entities.rs}}
```

Now for the map loading.

```rust
// map.rs
{{#include ../../../code/rust-sokoban-c02-04/src/map.rs}}
```

Finally, we'll move the systems code into their own files (RenderingSystem to rendering_system.rs and InputSystem to input_system.rs). It should just be a copy paste from main with some import removals, so go ahead and do that.

Now the interesting thing about systems is that it's a folder with multiple files inside. If we do nothing else and try to use `RenderingSystem` or `InputSystem` in main we will get some compilation failures. We will have to add a `mod.rs` file in the `systems` folder and tell Rust what we want to export out of this folder. All this bit is doing is it's telling Rust we want the outside world (the world out of this folder) to be able to access RenderingSystem and InputSystem types.


```rust
// systems/mod.rs
{{#include ../../../code/rust-sokoban-c02-04/src/systems/mod.rs}}
```

Awesome, now that we've done that here is how our simplified main file looks like. Notice the mod and use declarations after the imports, those are again telling Rust that we want to use those modules.

```rust
// main.rs
{{#include ../../../code/rust-sokoban-c02-04/src/main.rs}}
```

Feel free to run at this point, everything should work just the same, the only difference is now our code is much nicer and ready for more amazing Sokoban features.

> **_CODELINK:_**  You can see the full code in this example [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-04).


