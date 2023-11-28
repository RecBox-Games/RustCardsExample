use super::*;
use ggez::{
    graphics::{Canvas, DrawParam}, Context, GameResult,
};
use glam::Vec2;
use std::f32::consts::PI;

// TODO: move to sahred location
const CARD_IMG_WIDTH: f32 = 148.0;
const CARD_IMG_HEIGHT: f32 = 148.0;

// screen locations
const SPLAYED_CARD_DISTANCE: f32 = 30.0;



//////// Deck ////////
impl Deck {
    fn deck_height(&self) -> usize {
        return ((self.cards.len() + 5) / 6) as usize
    }

    fn facedown_card_offset(&self, n: usize) -> Vec2 {
        Vec2::new(0.0, n as f32 * -2.0)
    }
    
    pub fn draw(&self, canvas: &mut Canvas, location: Vec2, deck_res: &StandardDeckResources) {
        for i in 0..self.deck_height() {
            canvas.draw(deck_res.get_back_image(), location + self.facedown_card_offset(i));
        }
    }

    fn top_offset(&self) -> Vec2 {
        self.facedown_card_offset(self.deck_height())
    }
}

//////// MyCardGame ////////
impl MyCardGame {
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, deck_res: &StandardDeckResources) -> GameResult<()> {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        // draw splayed cards
        let splayed_cards_loc = Vec2::new( 0.0, 40.0 );
        for (i, card_spec) in self.splayed_cards.iter().enumerate() {
            let card_loc = splayed_cards_loc + Vec2::new(SPLAYED_CARD_DISTANCE * i as f32, 0.0);
            canvas.draw(deck_res.get_card_image(card_spec), card_loc);
        }
        //
        // draw center card
        let center_card_loc = Vec2::new(
            (screen_width - CARD_IMG_WIDTH) / 2.0,
            (screen_height - CARD_IMG_HEIGHT) / 2.0,
        );
        canvas.draw(deck_res.get_card_image(&self.center_card), center_card_loc);
        //
        // draw deck
        let deck_loc = center_card_loc + Vec2::new(CARD_IMG_WIDTH, 0.0);
        self.deck.draw(canvas, deck_loc, deck_res);
        //
        // draw in-motion card
        let splaying_card_start = deck_loc + self.deck.top_offset();
        let splaying_card_end = splayed_cards_loc + Vec2::new(SPLAYED_CARD_DISTANCE*self.splayed_cards.len() as f32, 0.0);
        self.draw_splaying_card(canvas, splaying_card_start, splaying_card_end, deck_res);
        //
        Ok(())
    }

    pub fn draw_splaying_card(&self, canvas: &mut Canvas, start_loc: Vec2, end_loc: Vec2, deck_res: &StandardDeckResources) {
        let (card_spec, splay_progress) = match &self.splaying_card {
            Some((cs, sp)) => (cs, sp),
            _ => {return}, // don't draw the splaying card if there is no card being splayed
        };
        //
        use SplayProgression::*;
        match splay_progress {
            Rise(p) => {
                let rising_loc = start_loc - p.progress() * Vec2::new(0.0, 12.0);
                canvas.draw(deck_res.get_back_image(), rising_loc);
            }
            Flip(p) => {
                let risen_loc = start_loc - Vec2::new(0.0, 12.0);
                let flip_scale = (p.progress() * PI).cos().abs();
                let flip_offset = (1.0 - flip_scale)*0.5*CARD_IMG_WIDTH;
                let flip_loc = risen_loc + Vec2::new(flip_offset, 0.0);
                let card_side = if p.progress() < 0.5 {
                    deck_res.get_back_image()
                } else {
                    deck_res.get_card_image(card_spec)
                };
                canvas.draw(card_side, DrawParam::default()
                            .dest(flip_loc)
                            .scale(Vec2::new(flip_scale, 1.0)));
            }
            Travel(p) => {
                let risen_loc = start_loc - Vec2::new(0.0, 12.0);
                let travel_loc = interpolate(risen_loc, end_loc, p.progress());
                canvas.draw(deck_res.get_card_image(card_spec), travel_loc);
            }
        }
    }

}

// linear interpolation between two points
fn interpolate(start_loc: Vec2, end_loc: Vec2, progress: f32) -> Vec2 {
    (1.0 - progress) * start_loc  +  progress * end_loc
}
