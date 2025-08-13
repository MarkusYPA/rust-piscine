/* use rand::Rng;

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
        Self::translate(rand::rng().random_range(0..=3))
    }

    pub fn translate(value: u8) -> Suit {
        [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade][value as usize]
    }
}

impl Rank {
    pub fn random() -> Rank {
        Self::translate(rand::rng().random_range(1..14))
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
} */

use rand::Rng;

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

impl Rank {
    #[inline]
    pub fn random() -> Self {
        Self::translate(rand::thread_rng().gen_range(1..14))
    }

    #[inline]
    pub fn translate(value: u8) -> Self {
        match value {
            1 => Self::Ace,
            n @ 2..=10 => Self::Number(n),
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => unreachable!(),
        }
    }
}

impl Suit {
    #[inline]
    pub fn random() -> Self {
        Self::translate(rand::thread_rng().gen_range(1..5))
    }

    #[inline]
    pub fn translate(value: u8) -> Self {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[inline]
pub fn winner_card(card: &Card) -> bool {
    card == &Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}