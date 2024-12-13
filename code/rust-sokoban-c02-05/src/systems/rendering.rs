use ggez::{
    conf,
    event::{self, KeyCode},
    graphics::{self, Color, DrawParam, Image},
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
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));

    // Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    // Iterate through all pairs of positions & renderables, load the image
    // and draw it at the specified position.
    for (_, (position, renderable)) in rendering_data.iter() {
        // Load the image
        let image = Image::from_path(context, renderable.path.clone()).unwrap();
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        // draw
        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        canvas.draw(&image, draw_params);
    }

    // Render any text
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;
    draw_text(context, &gameplay.state.to_string(), 525.0, 80.0);
    draw_text(context, &gameplay.moves_count.to_string(), 525.0, 100.0);

    // Finally, present the canvas, this will actually display everything
    // on the screen.
    canvas.finish(context).expect("expected to present");
}

pub fn draw_text(context: &mut Context, text_string: &str, x: f32, y: f32) {
    let text = graphics::Text::new(text_string);
    let destination = Vec2::new(x, y);
    let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
    let dimensions = Vec2::new(0.0, 20.0);

    graphics::queue_text(context, &text, dimensions, color);
    graphics::draw_queued_text(
        context,
        graphics::DrawParam::new().dest(destination),
        None,
        graphics::FilterMode::Linear,
    )
    .expect("expected drawing queued text");
}
