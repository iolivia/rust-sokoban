# Sound effects

In this section we will work on adding sound effects. In short, we want to play sounds in these circumstances:

1. when the player hits a wall or an obstacle - to let them know they cannot get through
1. when the player places a box on the correct spot - as an indication of "you've done it correctly"
1. when the player places a box on the incorrect spot - as an indication that the move was wrong

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

> **_CODELINK:_**  You can see the full code in this example [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-04).
