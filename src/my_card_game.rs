use crate::standard_deck::*;

mod draw_my_card_game;

use crate::progress::*;

use rand::seq::SliceRandom;
use rand::thread_rng;


struct Deck {
    cards: Vec<CardSpec>,
}

impl Deck {
    // randomized 52 cards
    fn new() -> Self {
        let mut cards: Vec<CardSpec> = Vec::new();
        for suit in CARD_SUITS.iter().map(|x| x.to_str()) {
            for rank in CARD_RANKS.iter().map(|x| x.to_str()) {
                let card_spec = CardSpec::from_strs(suit, rank);
                cards.push(card_spec);
            }
        }
        let mut deck = Self {
            cards
        };
        deck.shuffle();
        deck
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

const SPLAY_RISE_TIME: f32 = 0.5;
const SPLAY_FLIP_TIME: f32 = 0.5;
const SPLAY_TRAVEL_TIME: f32 = 1.0;
enum SplayProgression {
    Rise(Progression),
    Flip(Progression),
    Travel(Progression),
    //Lower(Progression),
}

/*
impl SplayProgression {
    fn new() -> Self {
        use SplayProgression::*;
        Rise(Progression::new(SPLAY_RISE_TIME))
    }

    fn update(&mut self) {
        use SplayProgression::*;
        match self {
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
    }

    fn is_done(&self) -> bool {
        match self {
            Travel(p) => p.is_done(),
            _ => false,
        }
    }
}
 */

// Notice that the members of MyCardGame have no data for graphics. This is
// so that the state of the game is separate from the representation (graphics)
// of the game. This is a personal design choice and you can do things
// differently if you please.
// - The "state" of the game is handled in this file and the "representation"
//   of the game is handled in draw_my_card_game.rs
pub struct MyCardGame {
    // deck: cards in the facedown deck in the center of the screen
    deck: Deck,
    // splayed_cards: cards at the top of the screen
    splayed_cards: Vec<CardSpec>,
    // splaying_card: the card traveling from the deck to the splayed_cards area
    splaying_card: Option<(CardSpec, SplayProgression)>,
    // center_card: card in the center of the screen next to the deck
    center_card: CardSpec,
}

impl MyCardGame {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        let center_card = deck.cards.pop().unwrap();
        Self {
            deck,
            splayed_cards: Vec::new(),
            splaying_card: None,
            center_card,
        }
    }
}


