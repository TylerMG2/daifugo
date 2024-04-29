use crate::player::{Player};
use crate::card::Card;
use crate::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub current_turn: usize,
    pub last_played: Option<Card>,
    pub first_round: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            deck: create_deck(),
            current_turn: 0,
            first_round: true,
            last_played: None,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn deal(&mut self) {
        // Shuffle the deck
        self.deck.shuffle(&mut thread_rng());

        // Deal the cards
        let player_count = self.players.len();
        for i in 0..self.deck.len() {
            self.players[i % player_count].add_card(self.deck[i].clone());
        }
    }
}

// Function to build a full 54 card deck
fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    for i in 0..13 {
        deck.push(Card::Suited { rank: i, suit: Suit::Spades });
        deck.push(Card::Suited { rank: i, suit: Suit::Hearts });
        deck.push(Card::Suited { rank: i, suit: Suit::Diamonds });
        deck.push(Card::Suited { rank: i, suit: Suit::Clubs });
    }
    for _ in 0..2 {
        deck.push(Card::Joker);
    }
    deck
}