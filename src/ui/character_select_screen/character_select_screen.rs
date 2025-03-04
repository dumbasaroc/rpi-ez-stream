use gtk4::glib;
use gtk4::*;

glib::wrapper! {

    pub struct CharacterSelectScreen(ObjectSubclass<imp::CharacterSelectScreen>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget,
                    gtk4::Orientable;

}

impl CharacterSelectScreen {
    pub fn new() -> Self {
        // Create new window
        let list: Vec<&'static str> = vec! [
            "Mario",
            "Donkey Kong",
            "Link",
            "Samus",
            "Kirby",
            "Fox",
            "Pikachu",
            "Yoshi"
        ];

        let css: Self = glib::Object::builder().build();
        css.initialize_character_list(list).unwrap();
        
        css
    }

    pub fn initialize_character_list<'a>(&self, characters: impl IntoIterator<Item = &'a str>) -> anyhow::Result<()> {
        use crate::ui::CharacterButton;

        let iter = characters.into_iter();
        self.character_box().remove_all();
        
        for character in iter {

            let button = CharacterButton::new(
                character,
                vec![ character ]
            );

            self.character_box().insert(&button, -1);
        }
        
        Ok(())
    }
}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(file = "./character_select_screen.ui")]
    #[properties(wrapper_type = super::CharacterSelectScreen)]
    pub struct CharacterSelectScreen {

        #[template_child]
        #[property(get)]
        search_bar: TemplateChild<SearchEntry>,

        #[template_child]
        #[property(get)]
        character_box: TemplateChild<FlowBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CharacterSelectScreen {

        const NAME: &'static str = "CharacterSelectScreen";
        type Type = super::CharacterSelectScreen;
        type ParentType = gtk4::Box;


        fn class_init(klass: &mut Self::Class) {
            crate::ui::CharacterButton::static_type();
            klass.bind_template();
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
