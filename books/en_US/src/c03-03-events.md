# Sounds and events

In this section we will work on adding events which will be later used for adding sound effects. In short, we want to play sounds in these circumstances:

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

Let's look at the event definitions, it should be something like this.

```rust
// events.rs
{{#include ../../../code/rust-sokoban-c03-03/src/events.rs}}
```

## Event queue resource

Now let's add a resource for the event queue. We will have various systems writing to this queue and one system (the event system) consuming this queue. It's basically a multiple producer single consumer model.

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-03/src/components.rs:events}}
```

## Sending events

Now that we have a way to enqueue events, let's add the two events we need in the input_system: EntityMoved and PlayerHitObstacle.

```rust
// input.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input.rs:run_input}}

    /// Code omitted
    /// ......
    /// ......
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input.rs:event_obstancle}}

    /// Code omitted
    /// ......
    /// ......
    ///               
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input.rs:event_moved}}
```

I've omitted some of the code in the original file for readability, but we are really just adding two lines in the right places to create the events and add them to the `events` vector.

Finally we need to add the events back into the world which we do at the end of the system.

```rust
// input.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input.rs:run_input}}

    /// Code omitted
    /// ......
    /// ......

{{#include ../../../code/rust-sokoban-c03-03/src/systems/input.rs:event_add}}
}
```

## Consuming events - event system

Now it's time to add a way to consume the events, which will be the events system. This system will contain the logic for what should happen when a specific event is received.

Let discuss how we will handle each event:

* Event::PlayerHitObstacle -> this is where the sound playing will go, but we'll come back to this when we add the audio bits
* Event::EntityMoved(EntityMoved { id }) -> this is where we will add the logic for checking if the entity that just moved is a box and whether it's on a spot or not
* Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) -> this is where the sound playing will go, but we'll come back to this when we add the audio bits

```rust
// systems/events.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/events.rs}}
```

The end of this system is important, processing an event could lead to another event created. So we must add events back into the world again.

> ***CODELINK:***  You can see the full code in this example [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-03).
