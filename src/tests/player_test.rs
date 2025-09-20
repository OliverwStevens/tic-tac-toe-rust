use super::super::Player;

#[test]
fn it_creates_a_new_player() {
  let player_name: String = String::from("Player");
  let player_marker: char = 'X';
  let player = Player::new(String::from("Player"),'X');

  //expect the name and marker to be correct
  assert_eq!(&player.name, &player_name);
  assert_eq!(&player.marker, &player_marker);

}