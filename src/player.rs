#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub marker: char,
}

impl Player {
    pub fn new(name: String, marker: char) -> Self {
        Player {
            name,
            marker, // Use shorthand syntax - cleaner!
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Player;

    #[test]
    fn it_creates_a_new_player() {
        let player = Player::new(String::from("Player"), 'X');
        
        // Expect the name and marker to be correct
        assert_eq!(player.name, "Player");
        assert_eq!(player.marker, 'X');
    }
}