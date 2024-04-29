use std::fmt;

// Suit is a bitfield enum (repr(u8) tells the compiler to use a u8 to store the enum)
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Spades = 0b0001,
    Hearts = 0b0010,
    Diamonds = 0b0100,
    Clubs = 0b1000,
}

impl Suit {
    pub fn from_bits(bits: u8) -> Option<Suit> {
        match bits {
            0b0001 => Some(Suit::Spades),
            0b0010 => Some(Suit::Hearts),
            0b0100 => Some(Suit::Diamonds),
            0b1000 => Some(Suit::Clubs),
            _ => None,
        }
    }
}

// A card could be a suited card or a joker
#[derive(Debug, Clone)]
pub enum Card {
    Suited { rank: usize, suit: Suit },
    Joker,
}

// Overriding the Display trait for Card and Suit
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::Suited { rank, suit } => {
                let rank_name = match rank {
                    8 => "Jack",
                    9 => "Queen",
                    10 => "King",
                    11 => "Ace",
                    12 => "2",
                    _ => return write!(f, "{} of {:?}", rank + 3, suit),
                };
                write!(f, "{} of {:?}", rank_name, suit)
            }
            Card::Joker => write!(f, "Joker"),
        }
    }
}