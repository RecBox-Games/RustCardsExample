use super::*;
use ggez::{
    graphics::{self, Canvas, DrawParam}, Context, GameResult,
};
use glam::Vec2;

// TODO: move to sahred location
const CARD_IMG_WIDTH: f32 = 148.0;
const CARD_IMG_HEIGHT: f32 = 148.0;

// screen locations
const MAX_SPLAYED_CARDS: usize = 5;



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
        let splayed_cards_x = (screen_width - CARD_IMG_WIDTH * MAX_SPLAYED_CARDS as f32) / 2.0;
        let splayed_cards_loc = Vec2::new( splayed_cards_x, 60.0 );
        //
        // draw center card
        let center_card_loc = Vec2::new(
            (screen_width - CARD_IMG_WIDTH) / 2.0,
            (screen_height - CARD_IMG_HEIGHT) / 2.0,
        );
        canvas.draw(deck_res.get_card_image(&self.center_card),
                    DrawParam::new().dest(center_card_loc));
        //
        // draw deck
        let deck_loc = center_card_loc + Vec2::new(CARD_IMG_WIDTH, 0.0);
        self.deck.draw(canvas, deck_loc, deck_res);
        //
        // draw in-motion card
        /*let splaying_card_start = deck_loc + self.deck.top_offset();
        let splaying_card_end = splayed_cards_loc + Vec2(CARD_IMG_WIDTH*self.splayed_cards.len());
        self.draw_splaying_card(canvas, splaying_card_start, splaying_card_end);
*/
        Ok(())
    }

    pub fn draw_splaying_card(&self, canvas: &mut Canvas, start_loc: Vec2, end_loc: Vec2) {
//        use SplayProgression::*;
/*        let splay_progress = if let Some((a, b)p) = self.splaying_card {
            1
        };
        match self.spl {
            Rise(p) => {
                p.update();
                if p.is_done() {
                    *self = Flip(Progression::new(SPLAY_FLIP_TIME)); 
                }
            }
            Flip(p) => {
                p.update();
                if p.is_done() {
                    *self = Flip(Progression::new(SPLAY_TRAVEL_TIME)); 
                }
            }
            Travel(p) => {
                p.update();
            }
            _ => {}
        }
        */
    }

}

