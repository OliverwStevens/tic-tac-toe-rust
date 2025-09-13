pub struct Game {
  pub grid: [[Option<char>; 3]; 3],
}

impl Game {
  pub fn new() -> Self {
      Game {
          grid: [[None; 3]; 3],
      }
  }
}