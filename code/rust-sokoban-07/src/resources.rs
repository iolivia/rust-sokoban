use ggez::event::KeyCode;
use specs::World;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}
