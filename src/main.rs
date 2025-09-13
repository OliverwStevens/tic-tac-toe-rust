mod game;
use game::Game;

mod player;

use player::Player;

fn main() {

    let players = [
        Player::new(String::from("Player 1"), 'X'),
        Player::new(String::from("Player 2"), 'O'),
    ];

    let game = Game::new();

    println!("{:?}", players);
    println!("{:?}", game.grid);

}