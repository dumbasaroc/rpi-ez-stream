use gtk4::glib;
use gtk4::prelude::EditableExt;
use gtk4::*;

glib::wrapper! {
    pub struct PlayerNameEntry(ObjectSubclass<imp::PlayerNameEntry>)
        @extends Entry, Widget,
        @implements Accessible, Buildable, ConstraintTarget, Editable;
}

impl PlayerNameEntry {

    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Sets the on-change callback to edit
    /// the internal data struct for a specific
    /// player.
    /// 
    /// @TODO Follow the example in
    /// CharacterSelectButton::on_click(),
    /// it handles this much more gracefully
    /// than we do here.
    /// 
    /// # Parameters
    /// - `player_id`: The string ID of the player
    ///   whose data this callback should edit.
    pub fn set_change_callback(&self, player_id: &'static str) {

        self.set_internal_player_id(player_id);

        self.connect_changed(move |f| {
            use crate::application_data::ApplicationStateAPI;

            ApplicationStateAPI::set_player_tag(player_id, f.text());
        });
    }
}

mod imp {

    use gtk4::glib::object_subclass;
    use gtk4::CompositeTemplate;
    use gtk4::glib;
    use gtk4::glib::*;
    use gtk4::glib::subclass::types::*;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::ObjectExt;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Default, CompositeTemplate, glib::Properties)]
    #[template(resource = "/edu/rpi/ezstream/ui/main_screen/player_name_entry.ui")]
    #[properties(wrapper_type = super::PlayerNameEntry)]
    pub struct PlayerNameEntry {

        #[property(get, set)]
        pub internal_player_id: Rc<RefCell<String>>
    }

    #[object_subclass]
    impl ObjectSubclass for PlayerNameEntry {
        const NAME: &'static str = "PlayerNameEntry";
        type Type = super::PlayerNameEntry;
        type ParentType = gtk4::Entry;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl PlayerNameEntry {}

    #[derived_properties]
    impl ObjectImpl for PlayerNameEntry {}
    impl WidgetImpl for PlayerNameEntry {}
    impl EntryImpl for PlayerNameEntry {}

}


#[allow(unused_imports)]
mod tests {

    use crate::MainApplication;
    use gtk4::prelude::EditableExt;
    use gtk_tester::*;

    create_test!{
        ensure_typing_updates_app_data,
        MainApplication,
        |win| {

            use crate::application_data::ApplicationStateAPI;
            use crate::playerid;

            let entry: &super::PlayerNameEntry = &win.main_screen().p1_name_input();
            
            let mut new_text: String = format!(
                "{}", entry.text().to_string()
            );
            let characters_to_add = "xyz123456";

            for character in characters_to_add.chars() {
                // append char to new_text
                new_text.push(character);
                entry.set_text(&new_text);
                assert!(ApplicationStateAPI::get_player_tag(playerid!(PLAYER1)) == new_text);
            }
        }
    }
}
