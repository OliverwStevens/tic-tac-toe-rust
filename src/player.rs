#[derive(Debug)]

pub struct Player {
  name: String,
  marker: char
}

impl Player {
  pub fn new(name: String, marker: char) -> Self {
    Player {
      name: name,
      marker: marker
    }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn marker(&self) -> &char {
    &self.marker
  }
}