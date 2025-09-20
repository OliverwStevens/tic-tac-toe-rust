use std::fmt;

use crate::Player;


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
    
    pub fn place_marker(&mut self, row: usize, col: usize, mark: &char) {
        self.grid[row][col] = Some(*mark);
    }

    pub fn over(&self) -> bool {
        self.grid.iter().all(|row| {
            row.iter().all(|cell| {
                matches!(cell, Some(_))
            })
        })
    }
    
    
    pub fn turn(&mut self, row: usize, col: usize) {
        let player_marker = self.current_player().marker;
        self.place_marker(row, col, &player_marker);
        self.turn_index += 1;
    }
}