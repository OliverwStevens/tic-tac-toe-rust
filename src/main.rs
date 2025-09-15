mod game;
use game::Game;

mod player;

use player::Player;

fn main() {

    let players = [
        Player::new(String::from("Player 1"), 'X'),
        Player::new(String::from("Player 2"), 'O'),
    ];

    let mut game = Game::new();

    println!("{:#?}", players);
    println!("{:#?}", game.grid);

    game.place_marker(0, 0, 'x');
    game.place_marker(0, 0, 'x');
    game.place_marker(0, 2, 'o');

}