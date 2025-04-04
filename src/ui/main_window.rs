use gtk4::gio::prelude::ActionMapExtManual;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::*;

use crate::application_data::switch_active_module;
use crate::playerid;
use crate::ui::actions;
use crate::ui::{CharacterSelectScreen, MainScreen};


/// The internal name for the MainScreen object in
/// the GTK hierarchy
const MAIN_SCREEN_NAME: &str = "mainscreen";

/// The internal name for the CharacterSelectScreen object in
/// the GTK hierarchy
const CHARACTER_SELECT_SCREEN_NAME: &str = "css";


glib::wrapper! {

    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Buildable;

}

impl MainWindow {

    /// Instantiates a new MainWindow for the inputted
    /// Application.
    /// 
    /// # Parameters
    /// - `app`: A reference to the gtk4::Application
    /// this window should be attached to.
    /// 
    /// # Returns
    /// A newly instantiated MainWindow object.
    pub fn new(app: &Application) -> Self {

        use crate::ui::main_screen as cmn;

        // Create new window
        let win: MainWindow = glib::Object::builder().property("application", app).build();
        win.instantiate_actions();

        win.main_screen().p1_name_input().set_change_callback(playerid!(PLAYER1));
        win.main_screen().p2_name_input().set_change_callback(playerid!(PLAYER2));

        cmn::instantiate_score_entry(&win.main_screen().p1_score_input(), playerid!(PLAYER1));
        cmn::instantiate_score_entry(&win.main_screen().p2_score_input(), playerid!(PLAYER2));

        // Instantiate the default active module
        win.change_module("res/modules/smash_ultimate_stock_icons");

        win
    }


    /// A helper function to change the active
    /// module in the MODULE_HANDLER singleton.
    /// This function additionally calls the two
    /// actions that reload the CSS and set the
    /// visibility of the CharacterSelect buttons
    /// on the main screen, as well as populate
    /// the default character of the module.
    /// 
    /// # Parameters
    /// - `module_path`: The absolute or relative
    /// path of the base folder where the new
    /// module is located.
    fn change_module<P>(&self, module_path: P) where P: ToString {
        use crate::application_data::{APPLICATION_STATE, MODULE_HANDLER};
        use crate::application_data::AlterApplicationDataState;
        use crate::playerid;

        let path = module_path.to_string();
        switch_active_module(Some(path));

        let module_state = MODULE_HANDLER.lock().unwrap();
        let mut app_state = APPLICATION_STATE.lock().unwrap();

        match module_state.as_ref() {
            Some(module) => {
                app_state.set_player_character_name(
                    playerid!(PLAYER1),
                    module.default_character.display_name.clone()
                );
                app_state.set_player_character_name(
                    playerid!(PLAYER2),
                    module.default_character.display_name.clone()
                );
            },
            None => {
                app_state.set_player_character_to_none(playerid!(PLAYER1));
                app_state.set_player_character_to_none(playerid!(PLAYER2));
            }
        };

        drop(module_state);
        drop(app_state);

        gtk4::prelude::WidgetExt::activate_action(
            self,
            format!("win.{}", actions::UPDATE_CHARACTER_BUTTON_VISIBILITY_ACTION_NAME).as_str(),
            None
        ).unwrap();

        gtk4::prelude::WidgetExt::activate_action(
            self,
            format!("win.{}", actions::INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME).as_str(),
            None
        ).unwrap();
    }


    /// Returns the currently created MainScreen
    /// for reading and writing purposes
    pub fn main_screen(&self) -> MainScreen {

        let correct_child: gtk4::Widget = match self.scene_switcher().child_by_name(MAIN_SCREEN_NAME) {
            Some(c) => c,
            None => {
                log::error!("Stack switcher could not find main scene with name \"{}\"!", MAIN_SCREEN_NAME);
                panic!();
            }
        };

        if !correct_child.is::<MainScreen>() {
            log::error!("The provided child with name \"{}\" cannot be cast into the MainScreen type!", MAIN_SCREEN_NAME);
            panic!();
        }

        correct_child.downcast().unwrap()
    }


    /// Returns the currently created Character
    /// Select Screen for reading and writing
    /// purposes.
    pub fn character_select_screen(&self) -> CharacterSelectScreen {
        let correct_child = match self.scene_switcher().child_by_name(CHARACTER_SELECT_SCREEN_NAME) {
            Some(c) => c,
            None => {
                log::error!("Stack switcher could not find main scene with name \"{}\"!", CHARACTER_SELECT_SCREEN_NAME);
                panic!();
            }
        };

        if !correct_child.is::<CharacterSelectScreen>() {
            log::error!("The provided child with name \"{}\" cannot be cast into the CharacterSelectScreen type!", CHARACTER_SELECT_SCREEN_NAME);
            panic!();
        }

        correct_child.downcast().unwrap()
    }


    /// A helper function that creates and populates
    /// all the actions under `crate::ui::actions`
    /// module into the main window.
    fn instantiate_actions(&self) {

        const MAIN_WINDOW_GROUP_PREFIX: &str = "win";

        // Add in the actions that we need buttons
        // and the like to call when they're activated.

        self.add_action_entries([
            actions::create_write_data_action(),
            actions::create_initialize_character_select_data_action(),
            actions::create_switch_to_css_action(),
            actions::create_switch_to_mainscreen_action(),
            actions::create_update_character_button_visibility_action(),
            actions::create_set_css_player_action(),
            actions::create_swap_p1_p2_data_action(),
        ]);
        
        // Connect the action to the update button.
        self.main_screen().update_button().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::WRITE_DATA_ACTION_NAME).as_str() )
        );

        // Connect "initialize CSS" action to a button
        self.main_screen().p1_character().connect_clicked(|button| {
            
            button.activate_action(
                format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::SET_CSS_PLAYER_ACTION_NAME).as_str(),
                Some(&playerid!(PLAYER1).to_variant())
            ).unwrap();

            button.activate_action(
                format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::SWITCH_TO_CSS_ACTION_NAME).as_str(),
                None
            ).unwrap();
        });

        self.main_screen().p2_character().connect_clicked(|button| {

            button.activate_action(
                format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::SET_CSS_PLAYER_ACTION_NAME).as_str(),
                Some(&playerid!(PLAYER2).to_variant())
            ).unwrap();

            button.activate_action(
                format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::SWITCH_TO_CSS_ACTION_NAME).as_str(),
                None
            ).unwrap();
        });

        self.main_screen().switch_data().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::SWAP_P1_P2_DATA_ACTION_NAME).as_str() )
        );

        self.character_select_screen().back_button().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::SWITCH_TO_MAINSCREEN_ACTION_NAME).as_str() )
        );
    }
}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::MainWindow)]
    #[template(file = "main_window.ui")]
    pub struct MainWindow {

        #[template_child]
        #[property(get)]
        pub scene_switcher: TemplateChild<Stack>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {

        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = gtk4::ApplicationWindow;


        fn class_init(klass: &mut Self::Class) {
            // Ensure child types are loaded at this point
            crate::ui::MainScreen::static_type();
            crate::ui::CharacterSelectScreen::static_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }




    impl ApplicationWindowImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl WidgetImpl for MainWindow {}

    #[glib::derived_properties]
    impl ObjectImpl for MainWindow {}
}
