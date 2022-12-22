use ggez::{
    graphics::{self, Canvas, Color, DrawParam, Image, InstanceArray},
    Context,
};
use glam::Vec2;
use itertools::Itertools;
use specs::{Join, Read, ReadStorage, System};
use std::collections::HashMap, 
use std::time::Duration;

use crate::constants::TILE_WIDTH;
use crate::resources::Time;
use crate::{components::*, resources::Gameplay};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = Vec2::new(x, y);
        let color = Color::new(0.0, 0.0, 0.0, 1.0);

        canvas.draw(
            &text,
            graphics::DrawParam::new().dest(destination).color(color),
        )
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> String {
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
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;

        // Clearing the screen (this gives us the backround colour)
        let mut canvas =
            Canvas::from_frame(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions.
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        // Iterate each of the renderables, determine which image path should be rendered
        // at which drawparams, and then add that to the rendering_batches.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image_path = self.get_image(renderable, time.delta);

            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            let z = position.z;

            // Add to rendering batches
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
                let image = Image::from_path(self.context, image_path).expect("expected image");
                let mut sprite_batch = InstanceArray::new(self.context, image);

                for draw_param in draw_params.iter() {
                    sprite_batch.push(*draw_param);
                }

                canvas.draw(&sprite_batch, DrawParam::new());
            }
        }
        // Render any text
        self.draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&mut canvas, &gameplay.moves_count.to_string(), 525.0, 120.0);
        let fps = format!("FPS: {:.0}", self.context.time.fps());
        self.draw_text(&mut canvas, &fps, 525.0, 160.0);

        // Finally, present the context, this will actually display everything
        // on the screen.
        canvas.finish(self.context).expect("expected to present");
    }
}
