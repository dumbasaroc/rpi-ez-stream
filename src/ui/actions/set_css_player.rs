use gtk4::prelude::*;
use gtk4::gio::ActionEntry;
use gtk4::glib::VariantClass;
use crate::ui;

use crate::ui::MainWindow;


pub const SET_CSS_PLAYER_ACTION_NAME: &str = "SETCSSPLAYERNAME";

pub fn create_set_css_player_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(SET_CSS_PLAYER_ACTION_NAME)
        .parameter_type(Some(&String::static_variant_type()))
        .activate(|win: &MainWindow, _, player_id| {
            
            // --- Get correct PlayerID to propogate ---
            let player_id: String = match player_id {
                Some(v) => {
                    match v.classify() {
                        VariantClass::String => { v.get().unwrap() },
                        _ => {
                            panic!( "Variant was not of String type!" );
                        }
                    }
                },
                None => {
                    panic!( "Attempted to pass nothing into SET_CSS_PLAYER." );
                }
            };

            // --- Propogate the new PlayerID to all CSS buttons ---

            for cbutton in &win.character_select_screen().character_box().observe_children() {

                if cbutton.is_err() { break; }
                let cbutton = cbutton.unwrap()
                    .downcast::<gtk4::FlowBoxChild>().unwrap();

                let cbutton = cbutton.child();
                
                if cbutton.is_none() { break; }
                let cbutton = cbutton.unwrap();

                if cbutton.is::<ui::CharacterButton>() {
                    let cbutton = cbutton.downcast::<ui::CharacterButton>().unwrap();
                    cbutton.set_player_id(player_id.clone());
                }

            }
        })
        .build()

}
