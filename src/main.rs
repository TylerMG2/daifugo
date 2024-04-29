use daifugo::{Card, Player, Role};

fn main() {
    let mut player = Player {
        hand: ([0; 13], 0),
        role: Role::Millionaire,
    };
    player.add_card(Card::Suited{ rank: 0, suit: daifugo::Suit::Spades });
    println!("{}", player.hand.0[0])
}
