#[allow(dead_code)]
pub trait AppStateDataAPI {

    type PlayerIDType;
    type CharacterDataType;

    fn get_player_tag(&mut self, id: Self::PlayerIDType) -> String;

    fn get_player_score(&mut self, id: Self::PlayerIDType) -> u32;
    
    fn get_player_character(&mut self, id: Self::PlayerIDType) -> Option<Self::CharacterDataType>;

    fn set_player_score(&mut self, id: Self::PlayerIDType, score: u32);

    fn set_player_tag<P>(&mut self, id: Self::PlayerIDType, tag: P) where P: Into<String>;

    fn set_player_character_name<C>(&mut self, id: Self::PlayerIDType, char_data: C) where C: Into<String>;

    fn set_player_character_costume(&mut self, id: Self::PlayerIDType, costume_id: u32);

    fn set_player_character_to_none(&mut self, id: Self::PlayerIDType);

    fn set_is_bestof(&mut self, is_bestof: bool);

    fn set_bestof_firstto_counter(&mut self, ctr: u32);

    fn set_tournament_name<C>(&mut self, name: C) where C: Into<String>;

    fn set_bracket_location<C>(&mut self, loc: C) where C: Into<String>;
}