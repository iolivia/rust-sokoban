use ggez::event::KeyCode;
use specs::World;
use std::collections::VecDeque;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: VecDeque<KeyCode>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}
