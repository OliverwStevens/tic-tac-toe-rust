use crate::Player;

// #[derive(Debug, Clone)]
pub struct Game {
    pub grid: [[Option<char>; 3]; 3],
    pub players: [Player; 2],
    turn_index: usize
}

impl Game {
    
    pub fn new(players: [Player; 2]) -> Self {
        Game {
            grid: [[None; 3]; 3],
            players,
            turn_index: 0
        }
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.turn_index % self.players.len()]
    }


    pub fn print_grid(&self) {
        for (i, row) in self.grid.iter().enumerate() {
            let mut row_str = String::new();
            for (j, cell) in row.iter().enumerate() {
                row_str.push(self.cell_character(cell));
                if j < row.len() - 1 {
                    row_str.push_str(" | ");
                }
            }
            println!("{}", row_str);
            if i < self.grid.len() - 1 {
                println!("---------");
            }
        }
    }

    pub fn cell_character(&self, cell: &Option<char>) -> char {
        match cell {
            Some(c) => *c,
            None => ' ',
        }
    }
    
    pub fn place_marker(&mut self, row: usize, col: usize, mark: &char) {
        self.grid[row][col] = Some(*mark);
    }

    pub fn over(&self) -> bool {
        self.grid_full() || self.is_won()
    }

    fn grid_full(&self) -> bool {
        self.grid.iter().all(|row| {
            row.iter().all(|cell| {
                matches!(cell, Some(_))
            })
        })
    }

    // Add win scenarios, set winner instance variable
    fn is_won(&self) -> bool {
        let diagonal_1 = [self.grid[0][0], self.grid[1][1], self.grid[2][2]];
        let diagonal_2 = [self.grid[0][2], self.grid[1][1], self.grid[2][0]];

        self.three_in_block(self.grid) || self.three_in_block(self.rotate_90_clockwise(&self.grid)) || self.three_in_row(diagonal_1) || self.three_in_row(diagonal_2)
    }

    fn three_in_block(&self, grid: [[Option<char>; 3]; 3]) -> bool {
        grid.iter().any(|&row| self.three_in_row(row))
    }
    fn three_in_row(&self, values: [Option<char>; 3]) -> bool {
        if let [Some(a), Some(b), Some(c)] = values {
            a.to_ascii_lowercase() == b.to_ascii_lowercase() && 
            b.to_ascii_lowercase() == c.to_ascii_lowercase()
        } else {
            false
        }
    }

    fn rotate_90_clockwise(&self, grid: &[[Option<char>; 3]; 3]) -> [[Option<char>; 3]; 3] {
        let mut new_grid = [[None; 3]; 3];
        
        for i in 0..3 {
            for j in 0..3 {
                new_grid[j][2 - i] = grid[i][j];
            }
        }
        
        new_grid
    }



    
    
    pub fn turn(&mut self, row: usize, col: usize) {
        let player_marker = self.current_player().marker;
        self.place_marker(row, col, &player_marker);
        self.turn_index += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::Game;

    use  super::Player;

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
      
        assert!(game.over())
      }
      
      #[test]
      fn it_returns_false_when_the_game_is_not_over() {
        let game: Game = create_new_game();
      
        assert!(!game.over())
      
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


      #[test]
      fn it_returns_true_when_three_are_in_a_row() {
        let game: Game = create_new_game();
        let values = [Some('X'), Some('X'), Some('X')];
        assert!(game.three_in_row(values));
      }

      #[test]
      fn it_returns_false_when_there_are_not_three_are_in_a_row() {
        let game: Game = create_new_game();
        let values = [Some('X'), Some('O'), Some('X')];
        assert!(!game.three_in_row(values));
      }

      #[test]
      fn it_returns_true_when_three_are_in_a_block() {
        let mut game: Game = create_new_game();
        game.grid = [[Some('X'), Some('X'), Some('X')], [None, None, None], [None, None, None]];
        assert!(game.three_in_block(game.grid));
      }

      #[test]
      fn it_returns_true_when_three_are_in_a_rotated_block() {
        let mut game: Game = create_new_game();
        game.grid = [[Some('X'), None, None], [Some('X'), None, None], [Some('X'), None, None]];
        assert!(game.three_in_block(game.rotate_90_clockwise(&game.grid)));
      }

      #[test]
      fn it_returns_false_when_three_are_not_in_a_rotated_block() {
        let mut game: Game = create_new_game();
        game.grid = [[Some('X'), Some('X'), Some('X')], [None, None, None], [None, None, None]];
        assert!(!game.three_in_block(game.rotate_90_clockwise(&game.grid)));
      }
}