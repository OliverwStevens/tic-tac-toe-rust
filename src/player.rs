#[derive(Debug)]

pub struct Player {
  pub name: String,
  pub marker: char
}

impl Player {
  pub fn new(name: String, marker: char) -> Self {
    Player {
      name: name,
      marker: marker
    }
  }
}