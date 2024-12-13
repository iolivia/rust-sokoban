use ggez::{
    conf, event,
    graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment},
    input::keyboard::{self, KeyCode},
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

    // ANCHOR: draw_gameplay_state
    // Render any text
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;
    draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
    draw_text(&mut canvas, &gameplay.moves_count.to_string(), 525.0, 100.0);
    // ANCHOR_END: draw_gameplay_state

    // Finally, present the canvas, this will actually display everything
    // on the screen.
    canvas.finish(context).expect("expected to present");
}

// ANCHOR: draw_text
pub fn draw_text(canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
    let mut text = Text::new(TextFragment {
        text: text_string.to_string(),
        color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
        scale: Some(PxScale::from(20.0)),
        ..Default::default()
    });

    canvas.draw(&text, Vec2::new(x, y));
}
// ANCHOR_END: draw_text
