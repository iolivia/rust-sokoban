# Animations
In this section we are going to look at adding animations to our game, we'll start with some basic ones but feel free to add more complex ones given the ideas in this tutorial. We'll add two animations: making the player blink and making the boxes jiggle slightly in place. 

## What is an animation?
An animation is simply a set of frames played at a specific time interval that gives the illusion of movement. Think of it like a video (a video is just a set of images played in sequence), but much lower framerate. 

For example, to get our player blinking we'll have three animation frames: 
1. our current player with the eyes open
1. player with eyes a little bit closed
1. player with eyes completely closed

If we play these three frames in sequence you'll notice it looks like the player is blinking. You can try this out by opening the images and shifting between them quickly on the image preview. 

There are a few gotchas on this: 
* the assets need to be done with a specific framerate in mind - for us we will go with 250 milliseconds, meaning we will play a new animation frame every 250ms, so we will have 4 frames per second
* the assets need to be consistent with each other - imagine we had two types of players which had different assets and different looking eyes, we would have to make sure that when we create the three frames mentioned above they would be consistent, otherwise the two players would blink at different rates
* designing assets for a lot of frames is a lot of work, so we'll try to keep our animations quite simple and stick to the key frames

## How will it work?
So how is this going to work in our existing Sokoban game? We'll have to:
1. Change our renderable component to allow multiple frames - we could also create a new renderable component that handles animated renderables and keep the one we have for static renderables, but it feels a bit cleaner to keep them together for now
1. Modify the player entity construction to take multiple frames
1. Keep track of time in our rendering loop - we'll discuss this one in more detail so don't worry if it's not obvious why we need to do this
1. Change the rendering system taking into account the number of frames, the time and the frame that is supposed to be rendered at a given time

## Assets
Let's add the new assets for the player, it should then look like this. Notice we created a convention to name the frames sequentially, this is not strictly necessary, but it will help us keep track of the order easily.

![Player 1](./images/player_1.png)
![Player 2](./images/player_2.png)
![Player 3](./images/player_3.png)

```
├── resources
│   └── images
│       ├── box_blue.png
│       ├── box_red.png
│       ├── box_spot_blue.png
│       ├── box_spot_red.png
│       ├── floor.png
│       ├── player_1.png
│       ├── player_2.png
│       ├── player_3.png
│       └── wall.png
```

## Renderable
Now let's update our renderable component to receive multiple frames, instead of having a single path, we'll have a list of paths, this should be pretty straightforward.

```rust
// components.rs
{{#include ../code/rust-sokoban-c03-02/src/components.rs:14:27}}
```


