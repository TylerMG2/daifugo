use daifugo::{Player, Role, Game};

fn main() {
    let mut game = Game::new();
    let player = Player::new(Role::Millionaire);
    game.add_player(player);
    let player = Player::new(Role::Millionaire);
    game.add_player(player);
    let player = Player::new(Role::Millionaire);
    game.add_player(player);
    let player = Player::new(Role::Millionaire);
    game.add_player(player);
    game.deal();
    if let Some(player) = game.players.first() {
        println!("Player 1's hand:");
        for card in player.hand() {
            println!("{}", card);
        }
    }
}
