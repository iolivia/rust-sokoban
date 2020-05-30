use crate::components::*;
use crate::constants::TILE_WIDTH;
use crate::resources::*;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::nalgebra as na;
use ggez::Context;
use specs::{Join, Read, ReadStorage, System};
use std::time::Duration;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = na::Point2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = na::Point2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> Image {
        println!("delta MS {}", delta.as_millis());
        println!("renderable.paths.len() {}", renderable.paths.len());

        let image_index = if renderable.paths.len() == 1 {
            // we only have one image, so we just return that
            0
        } else {
            // If we have multiple, we want to select the right one based on
            // how much time has passed since last time we rendered (delta time).
            // We wil split each second into 4 frames (a new frame every 250ms).
            // So if we have 4 frames, it will be 1, 2, 3, 4 (in one second)
            // So if we have 3 frames, it will be 1, 2, 3, 1 (in one second)
            let frame_index = (delta.as_millis() % 1000) / 250;
            println!("frame_index {}", frame_index);

            // frame_index will be a number between 0 and 4, if we have less than 4
            // frames we need to make sure that wraps around, so we mod by how many
            // paths we actually have.
            let path_index = (frame_index as usize) % renderable.paths.len();
            println!("path_index {}", path_index);

            path_index
        };

        let image_path = renderable.paths[image_index].clone();

        Image::new(self.context, image_path).expect("expected image")
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
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&a, &b| a.0.z.partial_cmp(&b.0.z).expect("expected comparison"));

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image = self.get_image(renderable, time.delta);
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Render any text
        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}
