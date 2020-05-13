# Gameplay

The player character is able to move and push boxes on the field. Many (but not all!) games have some kind of objective
for the player to achieve.  The objective for Sokoban-style games is typically to push boxes onto a goal spot.  There's
nothing stopping the player from doing this now, but the game also isn't checking for success.  The player might achieve
the objective without realizing it!  Let's update the game to check for the success state.

Let's think about what we'll need to add to this game to check for the success condition and to notify the user
when they've beaten the level:

- A `resource` for tracking the game state
    - Is the game in progress or completed?
    - How many turns has the player used?
- A `system` for checking if the user has completed ther objective
- A `system` for updating the number of moves made
- UI for reporting game state

## Gameplay Resource

We're choosing to use a `resource` to track game state because the game state is
not associated with a specific entity. Let's start by defining a `Gameplay` resource.

```rust
// resources.rs
{{#include ../code/rust-sokoban-08/src/resources.rs:38:43}}
```

`Gameplay` has two fields, `state` and `moves_count`, which are used to track the
current state of the game (is the game still in play, or has the player won?) and
the number of moves taken.  `state` is defined by an `enum`, defined like so:

```rust
// resources.rs
{{#include ../code/rust-sokoban-08/src/resources.rs:17:20}}
```

The eagle-eyed reader will note that we used a macro to derive the `Default` trait 
for `Gameplay`, but not for the `GameplayState` enum. If we want to use `Gameplay`
as a resource, it must implement `Default`.

So, what will we do? Since Rust macros can't derive `Default` for enums
automatically, we must implement `Default` for `Gameplay` ourselves.

```rust
// resources.rs
{{#include ../code/rust-sokoban-08/src/resources.rs:32:36}}
```

Having defined the resource, let's register it with the world.

```rust
// resources.rs
{{#include ../code/rust-sokoban-08/src/resources.rs:12:15}}
```

Now, when the game is started, the `Gameplay` resource will look like this:

```rust
Gameplay {
    state: GameplayState::Playing,
    moves_count: 0
}
```

## Step Counter System

We can increment `Gameplay`'s `moves_count` field to track the number of turns taken. We already have a system dealing with user input in `InputSystem`, so let's adapt that for this purpose.

Since we need to mutate the `Gameplay` resource, we need to register it with
`InputSystem` by adding `Write<'a, Gameplay>` to the `SystemData` type
definition.

```rust
// input_system.rs
{{#include ../code/rust-sokoban-08/src/systems/input_system.rs:0:25}}
        ...
```

Since we've already done the work to check if a player character will move in
response to keypress, we can use that to determine when to increment the step
counter.

```rust
// input_system.rs
        ...
{{#include ../code/rust-sokoban-08/src/systems/input_system.rs:87:105}}
```

## Gameplay System

Next, let's integrate this resource with 

## Gameplay UI

You can see the full code in this example [here](https://github.com/iolivia/rust-book/tree/master/code/rust-sokoban-08).