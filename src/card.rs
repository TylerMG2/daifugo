// Suit is a bitfield enum (repr(u8) tells the compiler to use a u8 to store the enum)
#[repr(u8)]
enum Suit {
    Spades = 0b0001,
    Hearts = 0b0010,
    Diamonds = 0b0100,
    Clubs = 0b1000,
}

// A card could be a suited card or a joker
enum Card {
    Suited { rank: usize, suit: Suit },
    Joker,
}

// Overriding the Display trait for Card and Suit
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::Suited { rank, suit } => write!(f, "{} of {:?}", rank, suit),
            Card::Joker => write!(f, "Joker"),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Spades => write!(f, "Spades"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Clubs => write!(f, "Clubs"),
        }
    }
}