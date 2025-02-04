pub struct PlayerData {

    /// Numerical score
    player_score: u32,

    /// Player name parts
    player_name: String,

    /// Player character data - can be
    /// none (especially if characters
    /// can't be selected.)
    player_character: Option<String>,
}


impl PlayerData {

    // Getters

    pub fn score(&self) -> u32 {
        self.player_score
    }

    pub fn name(&self) -> &str {
        &self.player_name
    }

    pub fn character(&self) -> Option<&str> {
        self.player_character.as_deref()
    }

    
    // Setters

    pub fn set_score(&mut self, score: u32) {
        self.player_score = score;
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.player_name = name.into();
    }

    pub fn set_character(&mut self, character: Option<impl Into<String>>) {
        self.player_character = match character {
            Some(s) => Some(s.into()),
            None => None
        };
    }

}


impl Default for PlayerData {

    fn default() -> Self {
        PlayerData {
            player_score: 0,
            player_name: format!(""),
            player_character: None
        }
    }
}
