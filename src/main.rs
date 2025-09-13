#[derive(Debug)]

struct Player {
    name: String,
    marker: char
}

struct Game {
    grid: [[Option<char>; 3]; 3],
}

impl Game {
    fn new() -> Self {
        Game {
            grid: [[None; 3]; 3],
        }
    }
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