use super::super::Game;
use super::super::Player;

fn create_new_game() -> Game {
  let players = [Player::new("Player 1".to_string(), 'X'), Player::new("Player 2".to_string(), 'O')];
  Game::new(players)
}
#[test]
fn it_creates_a_new_game_with_two_players() {
  let game: Game = create_new_game();
  assert_eq!(game.players.len(), 2)
}

#[test]
fn it_has_a_current_player() {
  let game: Game = create_new_game();

  assert_eq!(game.current_player().name, game.players[0].name)
}

#[test]
fn it_can_place_a_marker() {
  let mut game: Game = create_new_game();
  let marker = game.players[0].marker;
  game.place_marker(0, 0, &marker);
  assert_eq!(game.grid[0][0], Some(marker));

}

#[test]
fn it_returns_true_when_the_game_is_over() {
  let mut game: Game = create_new_game();
  game.grid = [[Some('X'); 3]; 3];

  assert_eq!(game.over(), true)
}

#[test]
fn it_returns_false_when_the_game_is_not_over() {
  let game: Game = create_new_game();

  assert_eq!(game.over(), false)

}

#[test]
fn it_returns_the_cell_character() {
  let mut game: Game = create_new_game();
  let marker = game.players[0].marker;
  game.grid[0][0] = Some(marker);

  assert_eq!(game.cell_character(&game.grid[0][0]), marker);
  assert_eq!(game.cell_character(&game.grid[1][0]), ' ');

}

#[test]
fn it_plays_a_turn_and_switches_players_appropriately() {
  let mut game: Game = create_new_game();
  game.turn(0, 0);
  assert_eq!(game.current_player().name, game.players[1].name);
  
  game.turn(1, 0);
  assert_eq!(game.current_player().name, game.players[0].name);


}