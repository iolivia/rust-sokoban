# Sound effects

In this section we will work on adding sound effects. In short, we want to play sounds in these circumstances:

1. when the player hits a wall or an obstacle - to let them know they cannot get through
1. when the player places a box on the correct spot - as an indication of "you've done it correctly"
1. when the player places a box on the incorrect spot - as an indication that the move was wrong

## Audio store

Now in order to play the sound the wav files need to be loaded. To avoid loading them on the fly every time before we play the sound we'll create an audio store and load them up at the beginning of the game.

We'll use a resource for the audio store.

```rust
{{#include ../../../code/rust-sokoban-c03-04/src/components.rs:audio_store}}
```

And let's add the code for initializing the store, which means pre-loading all the sounds needed for the game.

```rust
{{#include ../../../code/rust-sokoban-c03-04/src/map.rs:load_sounds}}
```

And then call this function when we are initializing the level.

```rust
{{#include ../../../code/rust-sokoban-c03-04/src/map.rs:initialize_level}}
```

## Playing audio

Finally, let's add the ability to play the sound in the store.

```rust
{{#include ../../../code/rust-sokoban-c03-04/src/components.rs:audio_store_impl}}
```

And now let's play in the event system.

```rust
// systems/events.rs
{{#include ../../../code/rust-sokoban-c03-04/src/systems/events.rs}}
```

Now let's run the game and enjoy those sound effects!

<video width="75%" controls>
    <source src="./videos/audio.mov" type="video/mp4">
</video>

> **_CODELINK:_**  You can see the full code in this example [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-04).
