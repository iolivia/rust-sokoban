# Input

It wouldn't be a game if we couldn't move the player, would it? In this section we'll be making the player move with the arrow keys.

## Input events
The first step for making our player move is to start listening to input events. If we take a quick look at the [ggez input example](https://github.com/ggez/ggez/blob/master/examples/input_test.rs#L59) we can see we can subscribe to all sort of mouse and keyboard related events, for now we probably only want `key_down_event`.

Let's add this code to `main.rs`.

```rust
impl event::EventHandler for Game {
    ...

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
    }
}
```

If we run this we should see the print lines in the console.

```
Key pressed: Left, modifier NONE, repeat: false
Key pressed: Left, modifier NONE, repeat: false
Key pressed: Right, modifier NONE, repeat: false
Key pressed: Up, modifier NONE, repeat: false
Key pressed: Down, modifier NONE, repeat: false
Key pressed: Left, modifier NONE, repeat: false
```

