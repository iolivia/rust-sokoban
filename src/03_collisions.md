# Collisions

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
        .with(Position {z: 10.0, ..position})
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
        .with(Position {z: 10.0, ..position})
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
        .with(Position {z: 10.0, ..position})
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .with(Movable)
        .build();
}
```


