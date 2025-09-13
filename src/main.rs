mod game;
use game::Game;

#[derive(Debug)]

struct Player {
    name: String,
    marker: char
}

fn main() {

    let players = [
        Player {name: String::from("Player 1"), marker: 'X'},
        Player {name: String::from("Player 2"), marker: 'O'}
    ];

    let game = Game::new();

    println!("{:?}", players);
    println!("{:?}", game.grid);

}