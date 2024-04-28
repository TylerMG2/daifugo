struct Player {
    hand: ( [u8; 13], u8) // A player's hand is a tuple of an array of 13 u8s and an i8 where the i8 is the number of jokers in the hand
}

impl Player {

    // Add a card to the player's hand
    fn add_card(&mut self, card: Card) {
        match card {
            Card::Suited { rank, suit } => {
                self.hand.0[rank] |= suit as u8
            },
            Card::Joker => self.hand.1 += 1,
        }
    }

    // Remove a card from the player's hand
    fn remove_card(&mut self, card: Card) {
        match card {
            // If the card is a suited card, remove it from the hand
            Card::Suited { rank, suit } => {
                // If the card is not in the hand, panic
                if (self.hand.0[rank] & suit as u8) == 0 {
                    panic!("No such card in hand")
                }
                self.hand.0[rank] &= !(suit as u8)
            },
            // If the card is a joker and the player has jokers in hand, remove one joker
            Card::Joker if self.hand.1 > 0 => {
                self.hand.1 -= 1
            },
            _ => panic!("No such card in hand"), // Panic if the player doesn't have the card
        }
    }
}