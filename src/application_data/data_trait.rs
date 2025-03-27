pub trait AlterApplicationDataState {

    type PlayerIDType;

    fn set_player_score(&mut self, id: Self::PlayerIDType, score: u32);

    fn set_player_tag<P>(&mut self, id: Self::PlayerIDType, tag: P) where P: Into<String>;

    fn set_player_character_name<C>(&mut self, id: Self::PlayerIDType, char_data: C) where C: Into<String>;

    fn set_player_character_costume(&mut self, id: Self::PlayerIDType, costume_id: u32);

    fn set_player_character_to_none(&mut self, id: Self::PlayerIDType);
}