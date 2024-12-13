# Moving the player

It wouldn't be a game if we couldn't move the player, would it? In this section we will figure out how to grab input events.

## Input events

The first step for making our player move is to start listening to input events. If we take a quick look at the [ggez input example](https://github.com/ggez/ggez/blob/master/examples/input_test.rs#L59) we can see we can check if a given key is pressed using `is_key_pressed`.

Let's start with a very basic implementation of the input system where we simply check if a key is pressed and print to the console.

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:input_system_print}}
```

Then, we'll add this code inside the `event::EventHandler` implementation block for our Game:

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:handler}}
```

If we run this we should see the print lines in the console.

```sh
LEFT
LEFT
RIGHT
UP
DOWN
LEFT
```

## Input system

Let's start by implementing the final input system now.

We already have a way to check if a key was pressed, now we need to implement the logic that will move the player. The logic we are aiming to implement:

* if UP is pressed, we move the player one position up on the y axis
* if DOWN is pressed, we move the player one position down on the y axis
* if LEFT is pressed, we move the player one position left on the x axis
* if RIGHT is pressed, we move the player one position right on the x axis

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:input_system}}
```

The input system is pretty simple, it grabs all the players and positions (we should only have one player but this code doesn't need to care about that, it could in theory work if we have multiple players that we want to control with the same input). And then for every player and position combination, it will grab the first key pressed and remove it from the input queue. It will then figure out what is the required transformation - for example if we press up we want to move one tile up and so on, and then applies this position update.

Pretty cool! Here's how it should look like. Notice we can go through walls and boxes. We'll fix that up in the next section when we add the movable component.

![Moving player](./images/input.gif)

> **_CODELINK:_**  You can see the full code in this example [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-02).
