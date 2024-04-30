use crate::Card;
use crate::Game;
use crate::Suit;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Role {
    Millionaire,
    Rich,
    MiddleClass,
    Poor,
    Beggar,
}

pub struct Player {
    hand: ( [u8; 13], u8), // A player's hand is a tuple of an array of 13 u8s and an i8 where the i8 is the number of jokers in the hand
    pub role: Role,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hand: ([0; 13], 0),
            role: Role::MiddleClass,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        match card {
            Card::Suited { rank, suit } => {
                self.hand.0[rank] |= suit as u8
            },
            Card::Joker => self.hand.1 += 1,
        }
    }

    // TODO: Return a Result instead of panicking
    pub fn remove_card(&mut self, card: Card) {
        match card {
            Card::Suited { rank, suit } => {
                if (self.hand.0[rank] & suit as u8) == 0 {
                    panic!("No such card in hand")
                }
                self.hand.0[rank] &= !(suit as u8)
            },
            Card::Joker if self.hand.1 > 0 => {
                self.hand.1 -= 1
            },
            Card::Joker => panic!("No such card in hand"),
        }
    }

    pub fn hand(&self) -> Vec<Card> {
        let mut hand = Vec::new();
        for (rank, &suit) in self.hand.0.iter().enumerate() {
            for i in 0..4 {
                let bit = 1 << i;
                if (suit & bit) != 0 {
                    if let Some(suit) = Suit::from_bits(bit) {
                        hand.push(Card::Suited { rank, suit });
                    }
                }
            }
        }
        for _ in 0..self.hand.1 {
            hand.push(Card::Joker);
        }
        hand
    }
}

pub trait Bot {
    fn next_turn(&self, game: &Game) -> Option<Vec<Card>>;
    fn get_player(&mut self) -> &mut Player;
}