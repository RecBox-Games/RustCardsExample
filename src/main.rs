//! A collection of semi-random shape and image drawing examples.

use ggez::{
    event,
    glam::Vec2,
    graphics,
    Context, GameResult,
};
use std::path;

struct MainState {
    card_img: graphics::Image,
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let card_img = graphics::Image::from_path(ctx, "/card_fronts/card_diamonds_07.png")?;

        let s = MainState {
            card_img,
        };

        Ok(s)
    }
}


impl event::EventHandler<ggez::GameError> for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw an image.
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        let dst = Vec2::new(20.0, 20.0);
        canvas.draw(&self.card_img, graphics::DrawParam::new().dest(dst));

        // Finished drawing, show it all on the screen!
        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);
    let (mut ctx, events_loop) = cb.build()?;
    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
