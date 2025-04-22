use gtk4::gio::prelude::ActionMapExtManual;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::*;

// use crate::application_data::switch_active_module;
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

impl Default for MainWindow {
    fn default() -> Self {
        glib::Object::builder().build()
    }
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
        use crate::ui::actions::*;
        use crate::application_data::ModuleHandlerAPI;

        // Create new window
        let win: MainWindow = glib::Object::builder().property("application", app).build();
        match win.load_fonts_from_directory("./res/fonts") {
            Ok(_) => {},
            Err(e) => {
                log::error!("Failed somewhere in loading fonts: {}", e);
            }
        };

        {
            let provider  = gtk4::CssProvider::new();
            provider.load_from_resource("/edu/rpi/ezstream/ui/main_window.css");
            gtk4::style_context_add_provider_for_display(
                &gdk::Display::default().expect("Could not connect to a display."),
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        win.instantiate_actions();

        win.main_screen().p1_name_input().set_change_callback(playerid!(PLAYER1));
        win.main_screen().p2_name_input().set_change_callback(playerid!(PLAYER2));

        cmn::instantiate_score_entry(&win.main_screen().p1_score_input(), playerid!(PLAYER1));
        cmn::instantiate_score_entry(&win.main_screen().p2_score_input(), playerid!(PLAYER2));

        // Instantiate the default active module
        
        ModuleHandlerAPI::load_module(&win, None::<String>).unwrap();
        // let _ = ModuleHandlerAPI::list_modules_in_folder();
        // ModuleHandlerAPI::load_module(&win, Some("res/modules/smash_ultimate_stock_icons")).unwrap();
        // win.change_module("res/modules/smash_ultimate_stock_icons");

        WidgetExt::activate_action(&win,
            format!("win.{}", SWITCH_TO_MAINSCREEN_ACTION_NAME).as_str(),
            None
        ).unwrap();

        win
    }


    /// Loads the '.ttf' files in the provided
    /// folder into the window's Pango context.
    fn load_fonts_from_directory(&self, directory: impl AsRef<std::path::Path>) -> std::io::Result<()> {

        const ALLOWED_EXTENSIONS: [&'static str; 2] = [ "ttf", "otf" ];

        let directory_contents = std::fs::read_dir(directory)?;
        for f in directory_contents {
            let f = f?;
            if f.metadata()?.is_file() && f.path().extension().is_some_and(
                |ext| { let ss = ext.to_str().unwrap(); ALLOWED_EXTENSIONS.contains(&ss) }
            ) {
                match self.pango_context().font_map() {
                    None => {
                        log::warn!("No available Pango FontMap for window... skipping.");
                        return Ok(());
                    },
                    Some(m) => {
                        match m.add_font_file(f.path()) {
                            Err(e) => {
                                log::warn!("Failed to load font file \"{}\".\n\nError: {}",
                                    f.file_name().to_str().unwrap(),
                                    e
                                );
                            },
                            Ok(_) => {
                                log::info!("Loaded font file \"{}\"", f.file_name().to_str().unwrap());
                            }
                        }
                    }
                };
            }
        }

        Ok(())
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
            actions::create_open_module_change_dialog_action(),
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
    #[template(resource = "/edu/rpi/ezstream/ui/main_window.ui")]
    pub struct MainWindow {

        #[template_child]
        #[property(get)]
        pub scene_switcher: TemplateChild<gtk4::Stack>
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
