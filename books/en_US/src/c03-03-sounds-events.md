# Sounds and events

In this section we will work on adding sound effects. In short, we want to play sounds in these circumstances: 
1. when the player hits a wall or an obstacle - to let them know they cannot get through
1. when the player places a box on the correct spot - as an indication of "you've done it correctly"
1. when the player places a box on the incorrect spot - as an indication that the move was wrong

Actually playing audio will not be too difficult as ggez provides this ability, but the biggest issue we have right now is that we need to determine *when* to play the sounds. 

Let's take the box on correct spot example. We could probably use our game state system and loop through the boxes and the spots and determine when we are in this state and play the sound. But that is not going to work because we will be interating many times per second, and we will always be in this state as long as the box doesn't move, so we will attempt to play many times per second, which is not what we want. We could try to keep some state to know what we are currently playing, but that doesn't feel right. The problem is we cannot do this by iteratively checking state, we instead need a reactive model where we can be told something has just happenned, and we need to take an action. What I've described here is an event model, we need to fire an event when a box gets placed on a spot, and then when we receive this event on the other end we need to play the sound. The really good thing about this is that we will be able to re-use this event system for many other purposes.

## Events infrastructure: How
Let's start by discussing how we will implement events. We will not use components or entities (although we could), instead we will use a resource very similar to the input queue. The parts of the code that need to enqueue events will need to get access to this resource, and we will then have a system which processes these events and take the appropriate actions.

## What events
Let's discuss in more detail what events we will need:
1. player hit obstacle - this can be an event in itself which we fire from the input system when we try to move but can't
1. box placed on correct/incorrect spot - we can model this as a single event with a property inside it that tells us if the box/spot combination is correct - thinking a bit deeper about how we can achieve this, we can have an entity moved event, and when we receive that event we can check the entity id of that entity that just moved to see if it's a box and if it's on the right/wrong/any spot (this is an example of creating an event chain - an event from an event)

## Events types
Now let's go into the implementation of events. We'll use an enum to define various event types. Now, we've used enums before (for the rendering type and the box colours) but this time we will take full advantage of the power of Rust enums. One of the most interesting things about them is that we actually attach properties to each enum variant. 

Let's look at our events enum.

```rust
// events.rs
{{#include ../../../code/rust-sokoban-c03-03/src/events.rs:13:23}}
```

Note the second `EntityMoved` and the second `BoxPlacedOnSpot`. Those are actually struct definitions where we can attach properties. Let's look at those structs now.

```rust
// events.rs
{{#include ../../../code/rust-sokoban-c03-03/src/events.rs:1:11}}
```

## Event queue resource
Now let's add a resource for the event queue. We will have various systems writing to this queue and one system (the event system) consuming this queue. It's basically a multiple producer single consumer model. 

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:54:57}}
```

And as always let's register this resource.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:14:18}}
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:20}}
```

## Sending events
Now that we have a way to enqueue events, let's add the two events we need in the input_system: EntityMoved and PlayerHitObstacle.

```rust
// input_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input_system.rs:1:42}}
                    // ...
                    // ...
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input_system.rs:83:124}}
```

I've omitted some of the code in the original file for readability, but we are really just adding two lines in the right places. 

## Consuming events - event system
Now it's time to add a way to consume the events, which will be the events system. This system will contain the logic for what should happen when a specific event is received.

Let discuss how we will handle each event:
* Event::PlayerHitObstacle -> this is where the sound playing will go, but we'll come back to this when we add the audio bits
* Event::EntityMoved(EntityMoved { id }) -> this is where we will add the logic for checking if the entity that just moved is a box and whether it's on a spot or not 
* Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) -> this is where the sound playing will go, but we'll come back to this when we add the audio bits

```rust
// event_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:1:34}}
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:36:63}}
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:71:78}}

```

## Audio assets
Now that we have the event bits in place, let's add audio assets. I've selected 3 sounds from this [asset pack](https://opengameart.org/content/512-sound-effects-8-bit-style), but feel free to select your own.

Correct sound [here](./sounds/correct.wav)

Incorrect sound [here](./sounds/incorrect.wav)

Wall sound [here](./sounds/wall.wav)

Let's add these sounds to a new folder under resources.

```
.
├── resources
│   ├── images
│   │   ├── box_blue_1.png
│   │   ├── box_blue_2.png
│   │   ├── box_red_1.png
│   │   ├── box_red_2.png
│   │   ├── box_spot_blue.png
│   │   ├── box_spot_red.png
│   │   ├── floor.png
│   │   ├── player_1.png
│   │   ├── player_2.png
│   │   ├── player_3.png
│   │   └── wall.png
│   └── sounds
│       ├── correct.wav
│       ├── incorrect.wav
│       └── wall.wav
├── Cargo.lock
└── Cargo.toml
```

## Audio store
Now in order to play the sound the wav files need to be loaded. To avoid loading them on the fly every time before we play the sound we'll create an audio store and load them up at the beginning of the game. 

We'll use a resource for the audio store.

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:6:9}}
```

And as always let's register this resource.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:14:20}}
```

And let's add the code for initializing the store.

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:21:32}}
```

## Playing audio
Finally, let's add the ability to play the sound in the store.

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:11:19}}
```

And now let's play in the event system.

```rust
    // event_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:24:37}}
                        // ...
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:61:73}}
```

Now let's run the game and enjoy those sound effects!

<video width="75%" controls>
    <source src="./videos/audio.mov" type="video/mp4">
</video>

> **_CODELINK:_**  You can see the full code in this example [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-03).
