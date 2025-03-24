use gtk4::glib::object::Cast;
use gtk4::prelude::WidgetExt;
use gtk4::glib;
use gtk4::prelude::{ButtonExt, ObjectExt};

glib::wrapper! {

    pub struct CharacterSelectScreen(ObjectSubclass<imp::CharacterSelectScreen>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget,
                    gtk4::Orientable;

}

impl CharacterSelectScreen {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /* GETTERS FOR WIDGETS NOT IMMEDIATELY IN TEMPLATE */
    
    pub fn back_button(&self) -> gtk4::Button {
        for child in self.top_bar().observe_children().into_iter() {
            if child.is_err() {
                continue;
            }

            let child = child.unwrap();
            if child.is::<gtk4::Button>() {
                let child = child.downcast::<gtk4::Button>().unwrap();
                return child;
            }
        }

        panic!("No such widget \"back_button\" in CSS.");
    }

    pub fn search_bar(&self) -> gtk4::SearchEntry {
        for child in self.top_bar().observe_children().into_iter() {
            if child.is_err() {
                continue;
            }

            let child = child.unwrap();
            if child.is::<gtk4::SearchEntry>() {
                let child = child.downcast::<gtk4::SearchEntry>().unwrap();
                return child;
            }
        }

        panic!("No such widget \"back_button\" in CSS.");
    }
}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    use crate::ui;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(file = "./character_select_screen.ui")]
    #[properties(wrapper_type = super::CharacterSelectScreen)]
    pub struct CharacterSelectScreen {

        #[template_child]
        #[property(get)]
        top_bar: TemplateChild<gtk4::Box>,

        #[template_child]
        #[property(get)]
        character_box: TemplateChild<FlowBox>,
    }

    #[gtk4::template_callbacks]
    impl CharacterSelectScreen {

        #[template_callback]
        fn on_type_character(flowbox: &FlowBox, searchbar: &SearchEntry) {
            let bar_text = searchbar.text().to_string();
            for child in &flowbox.observe_children() {
                if child.is_err() { break; }
                let child = child.unwrap()
                    .downcast::<gtk4::FlowBoxChild>().unwrap()
                    .child();
                
                if child.is_none() { break; }
                let child = child.unwrap();

                if child.is::<ui::CharacterButton>() {
                    let child = child.downcast::<ui::CharacterButton>().unwrap();
                    child.set_visible(
                        child.search_match(&bar_text)
                    );
                }
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CharacterSelectScreen {

        const NAME: &'static str = "CharacterSelectScreen";
        type Type = super::CharacterSelectScreen;
        type ParentType = gtk4::Box;


        fn class_init(klass: &mut Self::Class) {
            crate::ui::CharacterButton::static_type();
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl BoxImpl for CharacterSelectScreen {}
    impl WidgetImpl for CharacterSelectScreen {}

    #[glib::derived_properties]
    impl ObjectImpl for CharacterSelectScreen {}

}
