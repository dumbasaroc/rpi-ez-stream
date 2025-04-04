
/// # VALID IDENTIFIERS:
/// - `PLAYER1`
/// - `PLAYER2`
/// 
/// All else should result in compilation errors
#[macro_export]
macro_rules! playerid {
    (PLAYER1) => {
        "player_1"
    };

    (PLAYER2) => {
        "player_2"
    };
}


/// Takes in a String, returns a static
/// reference to a valid PlayerID.
pub fn get_playerid_from_string(s: String) -> &'static str {

    match s.as_str() {
        playerid!(PLAYER1) => playerid!(PLAYER1),
        playerid!(PLAYER2) => playerid!(PLAYER2),
        _ => { panic!( "Attempted to match \"{}\", which is an invalid PlayerID.", s ); }
    }

}
