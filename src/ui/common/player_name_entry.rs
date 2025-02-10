use gtk4::glib;
use gtk4::prelude::EditableExt;
use gtk4::*;

glib::wrapper! {
    pub struct PlayerNameEntry(ObjectSubclass<imp::PlayerNameEntry>)
        @extends Entry, Widget,
        @implements Accessible, Buildable, CellEditable, ConstraintTarget, Editable;
}

impl PlayerNameEntry {

    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Sets the on-change callback to edit
    /// the internal data struct for a specific
    /// player.
    /// 
    /// # Parameters
    /// - `player_id`: The string ID of the player
    /// whose data this callback should edit.
    pub fn set_change_callback(&self, player_id: &str) {

        self.set_s(player_id);

        self.connect_changed(|f| {
            use crate::application_data::APPLICATION_STATE;

            let mut lock = APPLICATION_STATE.lock().unwrap();
            let player = lock.get_player_via_id_mut(
                &f.s()
            );
            let p = match player {
                Some(p) => p,
                None => { return; }
            };

            p.set_name(f.text());
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
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default, CompositeTemplate, glib::Properties)]
    #[template(file = "player_name_entry.ui")]
    #[properties(wrapper_type = super::PlayerNameEntry)]
    pub struct PlayerNameEntry {

        #[property(get, set)]
        pub s: Rc<RefCell<String>>
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