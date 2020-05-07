# Project setup

Let's install [rustup](https://www.rust-lang.org/tools/install), this will install Rust and the Rust compiler for us. Now let's check everything is installed correctly using these two commands; the versions shouldn't matter too much so if yours are different don't worry about it.

```
$ rustc --version
rustc 1.40.0
$ cargo --version
cargo 1.40.0
```

## Creating a project

Cargo is Rust's package manager, and we will use it to create our game project. Change into a directory where you'd like the game to live and run the following command, with this we will be creating a new project called `rust-sokoban` using cargo. 

```
$ cargo init rust-sokoban
```

After the command has run you should see the following folder structure.  

```
- Cargo.toml
- src
-- main.rs
```

We can now run `cargo run` in this directory and we should see something like this.

```
$ cargo run
   Compiling rust-sokoban v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.30s
     Running `../rust-sokoban/target/debug/rust-sokoban`
Hello, world!
```

## Making it a game
It's time to make this basic hello world project into a game! We are going to be using [ggez](https://github.com/ggez/ggez) and the main reasons for this are: it's really beginner friendly, I have been using it a lot so I feel comfortable teaching something in it and it's actually quite good for simple 2D games.

Let add ggez as one of our dependencies in `Cargo.toml`.

```toml
[dependencies]
ggez = "0.5.1"
```

Then run `cargo run` again and you should see something like this. It should take slightly longer this time as it will be fetching these new dependencies from [crates.io](https://crates.io), then compiling them and finally linking them into our lib.

```
cargo run
    Updating crates.io index
    Downloaded ....
    ....
    Compiling ....
    ....
    Finished dev [unoptimized + debuginfo] target(s) in 2m 15s
    Running `.../rust-sokoban/target/debug/rust-sokoban`
    Hello, world!
```

Now let's actually use ggez in the main file and set up our window. Copy and paste this into the `main.rs` file and run again.

```rust
use ggez;
use ggez::{conf, event, Context, GameResult};

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update game logic here
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update draw here
        Ok(())
    }
}

pub fn main() -> GameResult {
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0));
    let (context, event_loop) = &mut context_builder.build()?;
    // Create the game state
    let game = &mut Game {};
    // Run the main event loop
    event::run(context, event_loop, game)
}

```

You should see something like this.

![Screenshot](./images/window.png)

Now let's walk through the concepts that we see in this first code in `main.rs`:
* **importing from dependencies** - hopefully this should be a familiar concept, but to bring types and namespaces into the scope from our dependent packages (or crates) we simply `use` them. for example: `use ggez;` to bring everything from the main ggez namespace
* **declaring a struct** - our Game struct which will hold our game state `struct Game {}`
* **implementing a Rust** [trait](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch10-02-traits.html) - think of a trait like an interface, in this case we are making `Game` implement the event::EventHandler interface that ggez provides `impl event::EventHandler for Game`
* **declaring a function** - `pub fn main() -> GameResult { ... }` this will declare a public function called main which takes no arguments and returns a `GameResult` (`GameResult` is a ggez specific object so don't worry about it too much for now, just know `Ok(())` means it was a successful result)
* **declaring a member function** - `fn update(&mut self, _context: &mut Context) -> GameResult { ... }` you might have noticed the `&mut self` on this line and wondering what that is about. `self` means this is a member function which belongs to a specific instance of a `Game`. `&mut` simply means that this function can change the state of the object, think of it as a non-const function in C++. We could write `&self` instead but then our `update` function would be pretty useless because it wouldn't be able to update any of the game state, which is what it's supposed to do

Hopefully that was not too scary of an intro and you're now super excited to learn more in the next chapter!