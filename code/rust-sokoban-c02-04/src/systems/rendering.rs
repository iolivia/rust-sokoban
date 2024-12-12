use ggez::{
    conf,
    event::{self, KeyCode},
    graphics::{self, DrawParam, Image},
    input::keyboard,
    Context, GameResult,
};
use glam::Vec2;
use hecs::{Entity, World};

use std::collections::HashMap;
use std::path;

use crate::components::*;
use crate::constants::*;

pub fn run_rendering(world: &World, context: &mut Context) {
    // Clearing the screen (this gives us the background colour)
    graphics::clear(context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

    // Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    // Iterate through all pairs of positions & renderables, load the image
    // and draw it at the specified position.
    for (_, (position, renderable)) in rendering_data.iter() {
        // Load the image
        let image = Image::new(context, renderable.path.clone()).expect("expected image");
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        // draw
        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        graphics::draw(context, &image, draw_params).expect("expected render");
    }

    // Finally, present the context, this will actually display everything
    // on the screen.
    graphics::present(context).expect("expected to present");
}
