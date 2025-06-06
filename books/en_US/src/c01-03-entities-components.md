# Components and entities

In this section we will create our components which we will then use in the hecs world.

## Defining components

Let's start by defining components. We previously discussed Position, Renderable and Movement - we'll skip movement for now. We will also need some components to identify each entity - for example we will need a Wall component so we can identify an entity as a wall by the fact that it has a wall component.

This should hopefully be straight-forward, the position components stores the x, y and z coordinates which will tell us where something is on the map, and the renderable component will receive a string path pointing to an image which we can render. All other components are marker components. The name marker component sounds pretty intimidating but it's essentially just a tag which has no other data fields.

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:components}}
```

## Creating entities

An entity is simply a numeric identifier tied to a set of components. So the way we'll create entities is by simply specifying which components they contain.

This is how entity creation looks now.

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:entities}}
```

## Assets

You might have noticed we are referencing the assets we will be using above in the entity creation. You are free to create your own assets or download the ones I am using which you can find right below (simply right click and save as image).

![Floor tile](./images/floor.png)
![Wall tile](./images/wall.png)
![Player tile](./images/player.png)
![Box tile](./images/box.png)
![Box tile](./images/box_spot.png)

Let's add the images to our project. We'll add a `resources` folder which will hold all our resources, for now this will only be images but in the future we will have other types of resources, like configuration files and/or audio files (keep going and you'll learn all about playing sounds in [Chapter 3.3 - Sounds and events](/c03-03-sounds-events.html)). We'll also add an `images` folder and place our pngs there, it should look like something like this. You can also use a different folder structure if you wish so, just make sure to use the right paths further down in this section when we'll be using the images.

```sh
├── resources
│   └── images
│       ├── box.png
│       ├── box_spot.png
│       ├── floor.png
│       ├── player.png
│       └── wall.png
├── src
│   └── main.rs
└── Cargo.toml
```

## World creation

Finally, let's tie everything together. We'll need to create a hecs::World object, we'll add that to our Game struct and we will initialize it first thing in our main. Here is the full code, running now should render the same blank window, but we've made tremendous progress in actually setting up our game components and entities! Next up, we'll get to rendering so we'll finally see something on screen!

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:main}}
```

Note that running now will report some warnings in the console about unused import(s) and/or fields, don't worry about these just yet as we'll fix them in the coming chapters.

> **_CODELINK:_**  You can see the full code in this example [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c01-03).
