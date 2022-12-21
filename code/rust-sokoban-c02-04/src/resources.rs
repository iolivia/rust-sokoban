use ggez::winit::event::VirtualKeyCode;
use specs::World;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<VirtualKeyCode>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}
