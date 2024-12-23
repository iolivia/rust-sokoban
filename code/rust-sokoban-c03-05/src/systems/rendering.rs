use ggez::{
    graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment}, Context,
};
use glam::Vec2;
use hecs::{Entity, World};
use itertools::Itertools;

use std::collections::HashMap;
use std::time::Duration;

use crate::components::*;
use crate::constants::*;

pub fn run_rendering(world: &World, context: &mut Context) {
    // Clearing the screen (this gives us the background colour)
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));

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
        let image_path = get_image(renderable, time.delta);
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
            let image = Image::from_path(context, image_path).unwrap();
            let mut mesh_batch = graphics::InstanceArray::new(context, Some(image));

            for draw_param in draw_params.iter() {
                mesh_batch.push(*draw_param);
            }

            canvas.draw(&mesh_batch, graphics::DrawParam::new());
        }
    }

    // Render any text
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;
    draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
    draw_text(&mut canvas, &gameplay.moves_count.to_string(), 525.0, 100.0);

    // Render FPS
    let fps = format!("FPS: {:.0}", context.time.fps());
    draw_text(&mut canvas, &fps, 525.0, 120.0);

    // Finally, present the canvas, this will actually display everything
    // on the screen.
    canvas.finish(context).expect("expected to present");
}

pub fn draw_text(canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
    let text = Text::new(TextFragment {
        text: text_string.to_string(),
        color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
        scale: Some(PxScale::from(20.0)),
        ..Default::default()
    });

    canvas.draw(&text, Vec2::new(x, y));
}

pub fn get_image(renderable: &Renderable, delta: Duration) -> String {
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
