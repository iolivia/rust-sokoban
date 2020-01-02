# Rust & ECS

You might have read the wikipedia page for Rust, in which case you would've seen this paragraph.
> Rust is a multi-paradigm system programming language focused on safety, especially safe concurrency. Rust is syntactically similar to C++, but is designed to provide better memory safety while maintaining high performance.

Now, since this sounds almost a bit too good to be true, how can Rust provide much better memory safety than its predecesors while maintaining high performance? Well, Rust provides an alternative memory management model (compared to garbage collection or manual management), which aims to avoid a lot of classical memory issues at compile time. The main idea is that the compiler uses this idea of ownership to enfore memory lifetime. (TODO more detail?) And as good as this is, it means that you have to make the compiler your friend to be able to write Rust fast and well and you might get occasionally stopped with really nice error messages. For this reason attempting to make a game in a more traditional inheritance based architecture can be especially painful in Rust. How can we make writing games in Rust nice and painless? Enter ECS.

ECS (Entity Component System) is an architectural pattern for writing games which follows the composition over inheritance principle. It's based on 3 main ideas:
* **Components** - data-only structs which hold different characteristics of entities: some examples of components: Position, Renderable, Movement, etc. The key part here is that this is pure data, no behaviour.
* **Entities** - entities are made up of multiple components, for example a player might be made up by Position, Renderable & Movement, while the floor might just be Position & Renderable since it doesn't move. Entities are pretty much just dummy containers of one or more components with a unique identifier.
* **Systems**: systems use entities and components and contain behaviour and logic based on that data. For example, you could have a rendering system which just iterates through all entities which contain renderable components and draws all of them. The key here is that the components themselves don't contain any behaviour, instead they use a system to interpret the data and act.

Because of this separation of data vs behaviour, ECS is a really good fit for Rust.

## How can we apply ECS to our game?



## How are we going to use ECS in Rust?
The good news is that there are a bunch of ECS crates in Rust, and we've chosen to use `specs` which is one of the most popular ones. 

Let's go ahead and declare our two components now.

The Position component will just have x and y coordinates.

```rust
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    x: f32,
    y: f32,
}
```

