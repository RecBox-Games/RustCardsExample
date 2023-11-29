use crate::standard_deck::*;

mod draw_my_card_game;

use crate::progress::*;
use ggez::input::keyboard::KeyCode;
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

const SPLAY_RISE_TIME: f32 = 0.2;
const SPLAY_FLIP_TIME: f32 = 0.4;
const SPLAY_TRAVEL_TIME: f32 = 1.5;
enum SplayProgression {
    Rise(Progression),
    Flip(Progression),
    Travel(Progression),
    //Lower(Progression),
}

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
                   *self = Travel(Progression::new(SPLAY_TRAVEL_TIME)); 
                }
            }
            Travel(p) => {
                p.update();
            }
        }
    }

    fn is_done(&self) -> bool {
        match self {
            SplayProgression::Travel(p) => p.is_done(),
            _ => false,
        }
    }
}


struct Player {
    handle: String,
    left_card: Option<CardSpec>,
    right_card: Option<CardSpec>,
}

impl Player {

    fn state_string(&self) -> String {
        let lcard_state = self.left_card.map_or("".to_string(), |x| x.to_string());
        let rcard_state = self.right_card.map_or("".to_string(), |x| x.to_string());
        format!("{}:{}", lcard_state, rcard_state)
    }
    
    fn update_controlpad(&self) {
        controlpads::send_message(&self.handle, &format!("state:{}", self.state_string()));
    }
}


// Notice that the members of MyCardGame have no data for graphics. This is
// so that the state of the game is separate from the representation (graphics)
// of the game. This is a personal design choice and you can do things
// differently if you please.
// - The "state" of the game is handled in this file and the "representation"
//   of the game is handled in draw_my_card_game.rs
pub struct MyCardGame {
    //// cards
    // deck: cards in the facedown deck in the center of the screen
    deck: Deck,
    // splayed_cards: cards at the top of the screen
    splayed_cards: Vec<CardSpec>,
    // splaying_cards: the card traveling from the deck to the splayed_cards area
    splaying_cards: Vec<(CardSpec, SplayProgression)>,
    // center_card: card in the center of the screen next to the deck
    center_card: CardSpec,
    // giving_card: facedown card that goes off the bottom of the screen to go to the player
    giving_card: Option<(CardSpec, Progression)>,
    ////
    //players: 
}

const GIVING_TRAVEL_TIME: f32 = 1.0;

impl MyCardGame {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        let center_card = deck.cards.pop().unwrap();
        Self {
            deck,
            splayed_cards: Vec::new(),
            splaying_cards: Vec::new(),
            center_card,
            giving_card: None,
        }
    }

    pub fn update(&mut self) {
        // update splaying card
        let mut i = 0;
        while i < self.splaying_cards.len() {
            let (card_spec, splay_p) = &mut self.splaying_cards[i];
            if splay_p.is_done() {
                self.splayed_cards.push(*card_spec);
                self.splaying_cards.remove(i);
            } else {
                splay_p.update();
                i += 1;
            };
        }
        // update giving card
        if let Some((card_spec, prog)) = &mut self.giving_card {
            prog.update();
            if prog.is_done() {
                self.giving_card = None;
            }
        }
    }

    fn deal(&mut self) {
        if let Some(next_card) = self.deck.cards.pop() {
            self.splaying_cards.push((next_card, SplayProgression::new()));
        }
    }

    fn start_give_card(&mut self) {
        if self.giving_card.is_some() {
            return;
        }
        if let Some(next_card) = self.deck.cards.pop() {
            self.giving_card = Some((next_card, Progression::new(GIVING_TRAVEL_TIME)));
        }
    }

    fn finish_give_card(&mut self) {
        //controlpads::send_message(
    }
    
    pub fn handle_key_press(&mut self, _key: KeyCode) {
        //self.deal();
        self.start_give_card();
    }

    pub fn handle_controlpad_message(&mut self, client: String, message: String) {
        if message.as_str() == "deal" {
            self.deal();
            controlpads::send_message(&client, "dealt")
                .unwrap_or_else(|e| println!("WARNING: Error sending controlpad message: {}", e));
        }
    }
    
}


