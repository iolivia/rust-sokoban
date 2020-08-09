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

// ANCHOR: rendering_system_impl
impl RenderingSystem<'_> {
    // ANCHOR_END: rendering_system_impl
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

    // ANCHOR: get_image
    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> Image {
        let path_index = match renderable.kind() {
            RenderableKind::Static => {
                // We only have one image, so we just return that
                0
            }
            RenderableKind::Animated => {
                // If we have multiple, we want to select the right one based on the delta time.
                // First we get the delta in milliseconds, we % by 1000 to get the seconds only
                // and finally we divide by 250 to get a number between 0 and 4. If it's 4
                // we technically are on the next iteration of the loop (or on 0), but we will let
                // the renderable handle this logic of wrapping frames.
                ((delta.as_millis() % 1000) / 250) as usize
            }
        };

        let image_path = renderable.path(path_index);

        Image::new(self.context, image_path).expect("expected image")
    }
    // ANCHOR_END: get_image
    // ANCHOR: rendering_system_end
}
// ANCHOR_END: rendering_system_end

// System implementation
// ANCHOR: system_impl
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );
    // ANCHOR_END: system_impl

    // ANCHOR: run_1
    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;
        // ANCHOR_END: run_1

        // Clearing the screen (this gives us the backround colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&a, &b| a.0.z.partial_cmp(&b.0.z).expect("expected comparison"));

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        // ANCHOR: run_for
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image = self.get_image(renderable, time.delta);
            // ANCHOR_END: run_for
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
            // ANCHOR: run_for_end
        }
        // ANCHOR_END: run_for_end

        // Render any text
        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
        // ANCHOR: run_end
    }
    // ANCHOR_END: run_end
    // ANCHOR: system_impl_end
}
// ANCHOR_END: system_impl_end
