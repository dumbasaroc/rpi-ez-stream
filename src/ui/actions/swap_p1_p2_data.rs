use gtk4::{gio::ActionEntry, prelude::EditableExt};

use crate::ui::MainWindow;


pub const SWAP_P1_P2_DATA_ACTION_NAME: &str = "SWAPP1P2";

pub fn create_swap_p1_p2_data_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(SWAP_P1_P2_DATA_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            let tmp_name = win.main_screen().p1_name_input().text();
            let tmp_score = win.main_screen().p1_score_input().value();

            win.main_screen().p1_name_input().set_text(win.main_screen().p2_name_input().text().as_str());
            win.main_screen().p1_score_input().set_value(win.main_screen().p2_score_input().value());

            win.main_screen().p2_name_input().set_text(&tmp_name);
            win.main_screen().p2_score_input().set_value(tmp_score);

            // @TODO Implement character switching here too

        })
        .build()

}


#[cfg(test)]
mod tests {
    use gtk4::prelude::{EditableExt, WidgetExt};
    use gtk_tester::create_test;
    use crate::MainApplication;

    create_test!{
        test_does_switch_properly_swap_data,
        MainApplication,
        |win| {
            use crate::ui::actions::SWAP_P1_P2_DATA_ACTION_NAME;
            use crate::application_data::ApplicationStateAPI;
            use crate::playerid;

            let p1_name = "p1";
            let p2_name = "p2";
            let p1_score: u32 = 0;
            let p2_score: u32 = 5;

            let main_screen = win.main_screen();

            main_screen.p1_name_input().set_text(p1_name);
            main_screen.p2_name_input().set_text(p2_name);
            main_screen.p1_score_input().set_value(p1_score as f64);
            main_screen.p2_score_input().set_value(p2_score as f64);

            win.activate_action(
                format!("win.{}", SWAP_P1_P2_DATA_ACTION_NAME).as_str(),
                None
            ).unwrap();

            // UI Changes
            assert!(main_screen.p1_name_input().text() == p2_name);
            assert!(main_screen.p2_name_input().text() == p1_name);
            assert!(main_screen.p1_score_input().value_as_int() == p2_score as i32);
            assert!(main_screen.p2_score_input().value_as_int() == p1_score as i32);

            // Internal State
            let curr_p1_name = ApplicationStateAPI::get_player_tag(playerid!(PLAYER1));
            let curr_p2_name = ApplicationStateAPI::get_player_tag(playerid!(PLAYER2));
            let curr_p1_score = ApplicationStateAPI::get_player_score(playerid!(PLAYER1));
            let curr_p2_score = ApplicationStateAPI::get_player_score(playerid!(PLAYER2));
            
            assert!(curr_p1_name == p2_name);
            assert!(curr_p2_name == p1_name);
            assert!(curr_p1_score == p2_score);
            assert!(curr_p2_score == p1_score);
        }
    }
}
