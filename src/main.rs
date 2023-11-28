use ggez::{
    event,
    glam::Vec2,
    graphics,
    Context, GameResult,
};

use std::{path, fs, collections::HashMap};

mod standard_deck;
use standard_deck::*;
use CardSuit::*;
use CardRank::*;
    

fn load_card_images(ctx: &mut Context) -> HashMap<CardSpec, graphics::Image> {
    let fronts_path = path::Path::new("./resources/card_fronts/");
    // panic if we can't read images in that directory
    let card_img_names: Vec<_> = fs::read_dir(fronts_path).unwrap().map(|x| {
        x.unwrap().file_name().to_str().unwrap().to_string()
    }).collect();
    let mut img_map: HashMap<CardSpec, graphics::Image> = HashMap::new();
    for img_name in card_img_names {
        let mut parts = img_name.split(".").next().unwrap().split("_");
        parts.next();
        let card_spec = CardSpec::from_strs(parts.next().unwrap(), parts.next().unwrap());
        // panic if we can't create each image        
        let img = graphics::Image::from_path(ctx, &format!("/card_fronts/{}", img_name)).unwrap();
        img_map.insert(card_spec, img);
        
    }
    img_map
}

struct MainState {
    card_fronts: HashMap<CardSpec, graphics::Image>,
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            card_fronts: load_card_images(ctx),
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
        canvas.draw(&self.card_fronts[&CardSpec::new(Spade, _03)], graphics::DrawParam::new().dest(dst));

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
