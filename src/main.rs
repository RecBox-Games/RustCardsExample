use ggez::{
    event,
    graphics,
    Context, GameResult,
};
use std::path;

mod progress;
mod my_card_game;
use my_card_game::*;
mod standard_deck;
use standard_deck::*;

struct MainState {
    deck_resources: StandardDeckResources,
    card_game: MyCardGame,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState {
            deck_resources: StandardDeckResources::new(ctx),
            card_game: MyCardGame::new(),
        };
        Ok(state)
    }
}


// By implementing EventHandler, we are making MainState play nicely with
// ggez's game loop. When we call event::run() in main with a MainState passed
// in, we are starting a loop where update() and draw() are called repeatedly
impl event::EventHandler<ggez::GameError> for MainState {

    // called once per frame (synchronous with MainState::draw())
    // default 60 frames per second
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    // called once per frame (synchronous with MainState::update())
    // default 60 frames per second
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.4, 0.9, 0.6, 1.0]));

        // make things pixely instead of blury
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        // draw MyCardGame
        self.card_game.draw(&mut canvas, ctx, &self.deck_resources)?;

        // finished drawing, show it all on the screen!
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
