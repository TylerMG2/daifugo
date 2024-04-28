// Represents a single card
struct Card {
    rank: usize,
    suit: Suit,
}

#[repr(u8)]
enum Suit {
    Spades = 0b0001,
    Hearts = 0b0010,
    Diamonds = 0b0100,
    Clubs = 0b1000,
}