use std::io;

mod game;
use game::Game;

mod player;

use player::Player;

fn main() {
  
  println!("Welcome to Rust Tic Tac Toe!");
  
  let player_1_name: String = player_name(1);
  
  let player_2_name: String = player_name(2);
  
  let players = [
  Player::new(player_1_name, 'X'),
  Player::new(player_2_name, 'O'),
  ];
  
  let mut game = Game::new(players);
  
  game.print_grid();

  while !game.over() {
    take_turn(&mut game);
  }

}

fn player_name(player_index: u8) -> String {
  println!("Player {}, input your nickname:", player_index);
  
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  println!("Your name is: {}", input.trim());
  input
}

fn take_turn(game: &mut Game) {
  println!("{}'s Turn:", game.current_player().name);
  
  match get_turn_input(&game) {
    Ok(value) => {
      game.turn(value.0, value.1);
      game.print_grid();

    }
    Err(e) => {
      println!("Error: {e}")
    }
  }
}
fn get_turn_input(game: &Game) -> Result<(usize, usize), io::Error> {
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  
  // Split and trim whitespace
  let parts: Vec<&str> = input
  .trim() // Remove leading/trailing whitespace
  .split_whitespace() // Split on any whitespace (spaces, tabs, newlines)
  .collect();
  
  // STRICT VALIDATION: Must be exactly 2 parts
  if parts.len() != 2 {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      match parts.len() {
        0 => "No input provided. Please enter two numbers (row col)",
        1 => "Only one number provided. Please enter two numbers (row col)",
        n if n > 2 => "Too many numbers! Please enter exactly two numbers (row col)",
        _ => "Invalid input format. Please enter two numbers (row col)",
      },
    ));
  }
  
  // Parse first part as row
  let row: usize = parts[0].parse().map_err(|e| {
    io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Invalid row '{}': {}", parts[0], e),
    )
  })?;
  
  // Parse second part as col
  let col: usize = parts[1].parse().map_err(|e| {
    io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Invalid column '{}': {}", parts[1], e),
    )
  })?;
  
  // Validate range (0-2 for tic-tac-toe)
  if row >= 3 || col >= 3 {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Row and column must be 0-2, got row={} col={}", row, col),
    ));
  }

  if game.grid[row][col].is_some() {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Cell already occupied")
    ));
  }
  
  Ok((row, col))
}
