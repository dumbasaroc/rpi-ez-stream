use log::warn;

#[derive(serde::Serialize)]
pub struct PlayerData {

    /// Numerical score
    player_score: u32,

    /// Player name parts
    player_name: String,

    /// Player character data - can be
    /// none (especially if characters
    /// can't be selected.)
    player_character: Option<CharacterData>,
}

#[derive(Clone, serde::Serialize)]
pub struct CharacterData {
    
    /// The name of the character, should match
    /// some internal hashmap @TODO TBD
    pub character_name: String,

    /// The costume index currently selected.
    pub costume_number: u32,
}

#[allow(dead_code)]
impl PlayerData {

    // Getters

    pub fn score(&self) -> u32 {
        self.player_score
    }

    pub fn name(&self) -> &str {
        &self.player_name
    }

    pub fn character(&self) -> Option<&CharacterData> {
        self.player_character.as_ref()
    }

    
    // Setters

    pub fn set_score(&mut self, score: u32) {
        self.player_score = score;
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.player_name = name.into();
    }

    pub fn set_character_costume(&mut self, costume: u32) {
        match &mut self.player_character {
            Some(c) => {
                c.costume_number = costume;
            },
            None => {
                warn!("Attempting to set {}'s NULL character's costume to {}!",
                    self.name(),
                    costume
                );
            }
        }
    }

    /// Sets the character to a specific character, or no character
    /// depending on the optional parameter.
    /// 
    /// # Note
    /// This method resets the costume to idx zero if given the
    /// name parameter.
    pub fn set_character<S>(&mut self, name: Option<S>)
        where S: Into<String>
    {
        let c = name.map(|n| CharacterData {
            character_name: n.into(),
            costume_number: 0
        });

        self.player_character = c;
    }

}


impl Default for PlayerData {

    fn default() -> Self {
        PlayerData {
            player_score: 0,
            player_name: String::new(),
            player_character: None
        }
    }
}
