# Moving the player

It wouldn't be a game if we couldn't move the player, would it? In this section we will figure out how to grab input events.

## Input events
The first step for making our player move is to start listening to input events. If we take a quick look at the [ggez input example](https://github.com/ggez/ggez/blob/master/examples/input_test.rs#L59) we can see we can subscribe to all sort of mouse and keyboard related events, for now we probably only want `key_down_event`.

Let's start listening to key events.

```rust
{{#include ../code/rust-sokoban-05/src/main.rs:153:160}}
{{#include ../code/rust-sokoban-05/src/main.rs:164}}
```

If we run this we should see the print lines in the console.

```
Key pressed: Left
Key pressed: Left
Key pressed: Right
Key pressed: Up
Key pressed: Down
Key pressed: Left
```

## Resources
Next up we'll add a resource, which is the specs way of sharing some state across systems which isn't part of your world. We'll use a resource for modelling the input queue of key presses, since that doesn't really fit into our existing components/entities model.

```rust
{{#include ../code/rust-sokoban-05/src/main.rs:47:50}}
```

And then we'll push the new key presses into the queue when `key_down_event` is called.

```rust
{{#include ../code/rust-sokoban-05/src/main.rs:153:164}}
```

Finally, we need to register the resources into specs like we did for components.

```rust
// Registering resources
{{#include ../code/rust-sokoban-05/src/main.rs:177:179}}

// Registering resources in main
{{#include ../code/rust-sokoban-05/src/main.rs:293:311}}
```

## Input system

Using this code we have a resource that is a continuous queue of input key presses. Next up, we'll start processing these inputs in a system.

```rust
{{#include ../code/rust-sokoban-05/src/main.rs:92:119}}
```

Finally we need to run the system in our update loop.

```rust
{{#include ../code/rust-sokoban-05/src/main.rs:133:141}}
```

The input system is pretty simple, it grabs all the players and positions (we should only have one player but this code doesn't need to care about that, it could in theory work if we have multiple players that we want to control with the same input). And then for every player and position combination, it will grab the first key pressed and remove it from the input queue. It will then figure out what is the required transformation - for example if we press up we want to move one tile up and so on, and then applies this position update.

Pretty cool! Here's how it should look like. Full code below.

![Moving player](./images/input.gif)

Notice we can go through walls and boxes. We'll fix that up in the next section when we add the movable component.

```rust
{{#include ../code/rust-sokoban-05/src/main.rs}}
```
