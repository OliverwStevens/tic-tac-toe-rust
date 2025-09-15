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
}

impl Game {
    pub fn new(players: [Player; 2]) -> Self {
        Game {
            grid: [[None; 3]; 3],
            players: players
        }
    }

    pub fn handle_marker(&self, row: usize, col: usize) -> Result<char> {
        if row >= 3 || col >= 3 {
            return Err(CellError);
        }
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

    pub fn place_marker(&mut self, row: usize, col: usize, mark: char) -> Result<()> {  // Fixed: Removed , CellError
        match self.handle_marker(row, col) {
            Ok(_) => {
                self.grid[row][col] = Some(mark);
                self.print_marker(Ok(mark));
                Ok(())
            }
            Err(e) => {
                self.print_marker(Err(e));  // No .clone() needed with Copy
                Err(e)
            }
        }
    }
}