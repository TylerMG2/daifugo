use daifugo::{Bot, Card, Game, Player};

pub struct ExampleBot(Player);

impl Bot for ExampleBot {
    fn next_turn(&self, _: &Game) -> Option<Vec<Card>> {
        None
    }
    fn get_player(&mut self) -> &mut Player {
        &mut self.0
    }
}

fn main() {
    let mut game = Game::new();
    game.add_bot(Box::new(ExampleBot(Player::new())));
    game.add_bot(Box::new(ExampleBot(Player::new())));
    game.add_bot(Box::new(ExampleBot(Player::new())));
    game.start();
    
    if let Some(bot) = game.players.first_mut() {
        let player = bot.get_player();
        for card in player.hand() {
            println!("{}", card);
        }
    }
}
