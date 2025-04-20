use gtk4::glib::object::Cast;
use gtk4::prelude::WidgetExt;
use gtk4::glib;
use gtk4::prelude::ObjectExt;

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
    
    /// Returns the button on the CSS that returns
    /// the user to the MainScreen.
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

        panic!("No such widget \"back_button\" in the CSS.");
    }


    /// Returns the FlowBox containing the character
    /// buttons.
    pub fn character_box(&self) -> gtk4::FlowBox {
        let child = self.scroll_window().child().unwrap();
        let viewport = child.downcast::<gtk4::Viewport>().unwrap();
        let child = viewport.child().unwrap();
        child.downcast::<gtk4::FlowBox>().unwrap()
    }


    /// Returns the SearchEntry on the Character
    /// Select Screen.
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

        panic!("No such widget \"search_bar\" in the CSS.");
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
    #[template(resource = "/edu/rpi/ezstream/ui/character_select_screen/character_select_screen.ui")]
    #[properties(wrapper_type = super::CharacterSelectScreen)]
    pub struct CharacterSelectScreen {

        #[template_child]
        #[property(get)]
        top_bar: TemplateChild<gtk4::Box>,

        #[template_child]
        #[property(get)]
        scroll_window: TemplateChild<gtk4::ScrolledWindow>,
    }

    #[gtk4::template_callbacks]
    impl CharacterSelectScreen {

        /// The callback for editing the searchbar
        /// on the CSS. This is responsible for filtering
        /// the characters based on the user's input into
        /// the searchbar.
        /// 
        /// # Parameters
        /// - `flowbox`: The area where all the character
        /// select buttons reside, used to check their
        /// data and hide/show them accordingly.
        /// - `searchbar`: The SearchEntry to pull text
        /// from.
        #[template_callback]
        fn on_type_character(flowbox: &FlowBox, searchbar: &SearchEntry) {
            let bar_text = searchbar.text().to_string();
            for child in &flowbox.observe_children() {
                if child.is_err() { break; }
                let flowbox_child = child.unwrap()
                    .downcast::<gtk4::FlowBoxChild>().unwrap();

                let child = flowbox_child.child();
                
                if child.is_none() { break; }
                let child = child.unwrap();

                if child.is::<ui::CharacterButton>() {
                    let child = child.downcast::<ui::CharacterButton>().unwrap();
                    flowbox_child.set_visible(
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
