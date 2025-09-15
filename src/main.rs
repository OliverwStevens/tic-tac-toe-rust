use std::io;

mod game;
use game::Game;

mod player;

use player::Player;

fn main() {

    // let players = [
    //     Player::new(String::from("Player 1"), 'X'),
    //     Player::new(String::from("Player 2"), 'O'),
    // ];

    // let mut game = Game::new();

    // println!("{:#?}", players);
    // println!("{:#?}", game.grid);

    // game.place_marker(0, 0, 'x');
    // game.place_marker(0, 0, 'x');
    // game.place_marker(0, 2, 'o');

    println!("Welcome to Rust Tic Tac Toe!");

    let player_1_name: String = player_name(1);

    let player_2_name: String = player_name(2);

    

}

fn player_name(player_index: u8) -> String {
    println!("Player {}, input your nickname:", player_index);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Your name is: {}", input.trim());
    input
}