use crate::player::Bot;
use crate::card::Card;
use crate::{Role, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Game {
    pub players: Vec<Box<dyn Bot>>,
    pub deck: Vec<Card>,
    pub current_turn: usize,
    pub last_played: Option<Card>,
    pub first_round: bool,
    roles : Vec<Role>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new(),
            deck: create_deck(),
            current_turn: 0,
            first_round: true,
            last_played: None,
            roles: Vec::new(),
        }
    }

    pub fn add_bot(&mut self, bot: Box<dyn Bot>) {
        self.players.push(bot);
    }

    fn deal(&mut self) {

        // Shuffle the deck
        self.deck.shuffle(&mut thread_rng());

        // Deal the cards
        let player_count = self.players.len();
        for i in 0..self.deck.len() {

            // 3 of diamonds starts the game
            let card = self.deck[i].clone();
            if card == (Card::Suited { rank: 0, suit: Suit::Diamonds }) {
                self.current_turn = i % player_count;
            }

            let player = self.players[i % player_count].get_player();
            player.add_card(card);
        }
    }

    pub fn start(&mut self) {

        // Shuffle players
        self.players.shuffle(&mut thread_rng());
        
        // Determine what roles are in play
        match self.players.len() {
            3 => {
                self.roles = vec![Role::Millionaire, Role::MiddleClass, Role::Beggar];
            },
            4 => {
                self.roles = vec![Role::Millionaire, Role::Rich, Role::Poor, Role::Beggar];
            },
            5 => {
                self.roles = vec![Role::Millionaire, Role::Rich, Role::MiddleClass, Role::Poor, Role::Beggar];
            },
            _ => panic!("Only supports 3-5 players!"),
        }

        self.deal();


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