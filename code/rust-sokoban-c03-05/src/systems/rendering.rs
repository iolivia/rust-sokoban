use ggez::{
    conf,
    event::{self, KeyCode},
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam, Image},
    input::keyboard,
    timer, Context, GameResult,
};
use glam::Vec2;
use hecs::{Entity, World};
use itertools::Itertools;

use std::collections::HashMap;
use std::path;
use std::time::Duration;

use crate::components::*;
use crate::constants::*;

pub fn run_rendering(world: &World, context: &mut Context) {
    // Clearing the screen (this gives us the background colour)
    graphics::clear(context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

    // Get time
    let mut query = world.query::<&Time>();
    let time = query.iter().next().unwrap().1;

    // Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

    // Iterate each of the renderables, determine which image path should be rendered
    // at which drawparams, and then add that to the rendering_batches.
    for (_, (position, renderable)) in rendering_data.iter() {
        // Load the image
        let image_path = get_image(context, renderable, time.delta);
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;
        let z = position.z;

        // draw
        let draw_param = DrawParam::new().dest(Vec2::new(x, y));
        rendering_batches
            .entry(z)
            .or_default()
            .entry(image_path)
            .or_default()
            .push(draw_param);
    }

    // Iterate spritebatches ordered by z and actually render each of them
    for (_z, group) in rendering_batches
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
    {
        for (image_path, draw_params) in group {
            let image = Image::new(context, image_path).expect("expected image");
            let mut sprite_batch = SpriteBatch::new(image);

            for draw_param in draw_params.iter() {
                sprite_batch.add(*draw_param);
            }

            graphics::draw(context, &sprite_batch, graphics::DrawParam::new())
                .expect("expected render");
        }
    }

    // Render any text
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;
    draw_text(context, &gameplay.state.to_string(), 525.0, 80.0);
    draw_text(context, &gameplay.moves_count.to_string(), 525.0, 100.0);

    // Render FPS
    let fps = format!("FPS: {:.0}", timer::fps(context));
    draw_text(context, &fps, 525.0, 120.0);

    // Finally, present the context, this will actually display everything
    // on the screen.
    graphics::present(context).expect("expected to present");
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

pub fn get_image(context: &mut Context, renderable: &Renderable, delta: Duration) -> String {
    let path_index = match renderable.kind() {
        RenderableKind::Static => {
            // We only have one image, so we just return that
            0
        }
        RenderableKind::Animated => {
            // If we have multiple, we want to select the right one based on the delta time.
            // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
            // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
            // we technically are on the next iteration of the loop (or on 0), but we will let
            // the renderable handle this logic of wrapping frames.
            ((delta.as_millis() % 1000) / 250) as usize
        }
    };

    renderable.path(path_index)
}
