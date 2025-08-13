//use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        // Self::translate(rand::rng().random_range(1..=3))
        Self::translate(random_range(0..=3))
    }

    pub fn translate(value: u8) -> Suit {
        [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade][value as usize]
    }
}

impl Rank {
    pub fn random() -> Rank {
        //Self::translate(rand::rng().random_range(1..=13))
        Self::translate(random_range(1..=13))
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..11 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("incorrect rank"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card == &Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}

fn random_range(range: std::ops::RangeInclusive<u8>) -> u8 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % (range.end() - range.start() + 1) as u32) as u8 + range.start()
}
