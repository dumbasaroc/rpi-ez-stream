use gtk4::glib;
use gtk4::prelude::{ButtonExt, WidgetExt};

glib::wrapper! {

    pub struct CharacterButton(ObjectSubclass<imp::CharacterButton>)
        @extends gtk4::Button, gtk4::Widget,
        @implements gtk4::Actionable, gtk4::Buildable;

}

impl CharacterButton {
    pub fn new<S, I>(character_name: S, aliases: I) -> Self where
        std::string::String: From<S>,
        I: IntoIterator<Item = String>
    {
        let button: Self = glib::Object::builder().build();
        
        let internal_character_name: String = character_name.into();
        button.set_character_name_internal(internal_character_name);
        button.set_aliases(aliases.into_iter().map(|s| s.to_string()).collect::<Vec<String>>());
        button.set_label(button.character_name_internal().as_str());

        button
    }


    /// Checks a given search query against both
    /// the internal name of the button, as well
    /// as any aliases the character may have.
    /// 
    /// # Parameters
    /// - `search_query`: The actual text the user
    /// inputted.
    /// 
    /// # Returns
    /// `true` if the query matched any string found
    /// in this CharacterButton, `false` if it did not.
    pub fn search_match<P>(&self, search_query: P) -> bool where std::string::String: From<P> {
        let str_search_query: String = search_query.into();
        if str_search_query == "" {
            return true;
        }

        if self.character_name_internal().to_lowercase().contains(&str_search_query.to_lowercase()) { return true; }

        for alias in self.aliases() {
            if alias.to_lowercase().contains(&str_search_query.to_lowercase()) { return true; }
        }

        false
    }
}

#[gtk4::template_callbacks]
impl CharacterButton {
    
    /// Handles changing a given player's character
    /// when the button is clicked.
    /// 
    /// # Parameters
    /// - `button`: The CharacterButton that
    /// was just clicked.
    #[template_callback]
    fn on_click(button: &CharacterButton) {
        use crate::application_data::ApplicationStateAPI;

        ApplicationStateAPI::set_player_character_name(
            crate::application_data::get_playerid_from_string(button.player_id()),
            button.character_name_internal()
        );
        
        button.activate_action(
            format!("win.{}", crate::ui::actions::SWITCH_TO_MAINSCREEN_ACTION_NAME).as_str(),
            None
        ).unwrap();
    }
}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(resource = "/edu/rpi/ezstream/ui/character_select_screen/character_select_button.ui")]
    #[properties(wrapper_type = super::CharacterButton)]
    pub struct CharacterButton {

        #[property(get, set)]
        character_name_internal: Rc<RefCell<String>>,

        #[property(get, set)]
        aliases: Rc<RefCell<Vec<String>>>,

        #[property(get, set)]
        player_id: Rc<RefCell<String>>,
    }
    
    #[glib::object_subclass]
    impl ObjectSubclass for CharacterButton {

        const NAME: &'static str = "CharacterButton";
        type Type = super::CharacterButton;
        type ParentType = gtk4::Button;


        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            super::CharacterButton::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ButtonImpl for CharacterButton {}
    impl WidgetImpl for CharacterButton {}

    #[glib::derived_properties]
    impl ObjectImpl for CharacterButton {}

}
