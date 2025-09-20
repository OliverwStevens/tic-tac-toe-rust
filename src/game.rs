use std::fmt;

use crate::Player;

type Result<T> = std::result::Result<T, CellError>;

#[derive(Debug, Clone, Copy)]  // Added Copy for cheap cloning of errors
pub struct CellError;

impl fmt::Display for CellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cell is invalid or out of bounds")
    }
}

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
    pub fn handle_marker(&self, row: usize, col: usize) -> Result<char> {
        match self.grid[row][col] {
            Some(_) => Err(CellError),
            None => Ok(' '),
        }
    }

    pub fn print_marker(&self, result: Result<char>) {
        match result {
            Ok(marker) => println!("Marker is {}", marker),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn place_marker(&mut self, row: usize, col: usize, mark: &char) -> Result<()> {
        match self.handle_marker(row, col) {
            Ok(_) => {
                self.grid[row][col] = Some(*mark);
                self.print_marker(Ok(*mark));
                Ok(())
            }
            Err(e) => {
                self.print_marker(Err(e));
                Err(e)
            }
        }
    }

    pub fn over(&self) -> bool {
        self.grid.iter().all(|row| {
            row.iter().all(|cell| {
                matches!(cell, Some(_))
            })
        })
    }
    
    
    pub fn turn(&mut self, row: usize, col: usize) -> Result<()> {
        let player_marker = self.current_player().marker;
        match self.place_marker(row, col, &player_marker) {
            Ok(_) => {
                self.turn_index += 1;
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}