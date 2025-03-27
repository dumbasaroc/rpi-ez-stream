
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
