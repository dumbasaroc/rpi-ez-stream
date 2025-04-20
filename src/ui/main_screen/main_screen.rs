use gtk4::glib;

glib::wrapper! {

    pub struct MainScreen(ObjectSubclass<imp::MainScreen>)
        @extends gtk4::Grid, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget,
                    gtk4::Orientable;

}

impl MainScreen {
    pub fn new() -> Self {
        glib::Object::builder().build()
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
    #[template(resource = "/edu/rpi/ezstream/ui/main_screen/main_screen.ui")]
    #[properties(wrapper_type = super::MainScreen)]
    pub struct MainScreen {

        #[template_child]
        #[property(get)]
        p1_character: TemplateChild<Button>,

        #[template_child]
        #[property(get)]
        p2_character: TemplateChild<Button>,

        #[template_child]
        #[property(get)]
        switch_data: TemplateChild<Button>,

        #[template_child]
        #[property(get)]
        update_button: TemplateChild<Button>,

        #[template_child]
        #[property(get)]
        p1_name_input: TemplateChild<ui::PlayerNameEntry>,

        #[template_child]
        #[property(get)]
        p2_name_input: TemplateChild<ui::PlayerNameEntry>,

        #[template_child]
        #[property(get)]
        p1_score_input: TemplateChild<ui::ScoreEntry>,

        #[template_child]
        #[property(get)]
        p2_score_input: TemplateChild<ui::ScoreEntry>,

        #[template_child]
        #[property(get)]
        tournament_info_bar: TemplateChild<ui::TournamentInfoBar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainScreen {

        const NAME: &'static str = "MainScreen";
        type Type = super::MainScreen;
        type ParentType = gtk4::Grid;


        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl GridImpl for MainScreen {}
    impl WidgetImpl for MainScreen {}

    #[glib::derived_properties]
    impl ObjectImpl for MainScreen {}

}

