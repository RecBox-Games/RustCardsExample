#[derive(Eq, Hash, PartialEq)]
pub enum CardSuit {
    Heart,
    Diamond,
    Spade,
    Club
}
use CardSuit::*;
impl CardSuit {
    pub fn from_str(s: &str) -> Self {
        match s {
            "hearts" => Heart,
            "diamonds" => Diamond,
            "spades" => Spade,
            "clubs" => Club,
            _ => panic!("bad suit: {}", s)
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum CardRank {
    _02,
    _03,
    _04,
    _05,
    _06,
    _07,
    _08,
    _09,
    _10,
    _J,
    _Q,
    _K,
    _A,
}
use CardRank::*;
impl CardRank {
    pub fn from_str(s: &str) -> Self {
        match s {
            "02" => _02,
            "03" => _03,
            "04" => _04,
            "05" => _05,
            "06" => _06,
            "07" => _07,
            "08" => _08,
            "09" => _09,
            "10" => _10,
            "J" => _J,
            "Q" => _Q,
            "K" => _K,
            "A" => _A,
            _ => panic!("bad rank: {}", s)
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct CardSpec {
    suit: CardSuit,
    rank: CardRank,
}
impl CardSpec {
    pub fn new(suit: CardSuit, rank: CardRank) -> Self {
        Self {
            suit,
            rank,
        }
    }
    
    pub fn from_strs(suit: &str, rank: &str) -> Self {
        Self {
            suit: CardSuit::from_str(suit),
            rank: CardRank::from_str(rank),
        }
    }
}
