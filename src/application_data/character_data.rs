use std::path::PathBuf;

pub struct ModuleHandler {
    current_module_name: String,
    base_directory_path: PathBuf,
    characters: Vec<CharacterData>,
    default_character: CharacterData,
}

impl ModuleHandler {

    pub fn new<P>(path: P) -> Self where P: AsRef<String> {
        let mut new_module = Self {
            current_module_name: "Something".to_string(),
            base_directory_path: PathBuf::from(path.as_ref()),
            characters: vec![],
            default_character: CharacterData {
                character_name: "Nil".to_string(),
                num_costumes: 0,
                aliases: vec![]
            }
        };

        // @TODO Finish this logic here, we should load some kind of
        // module data from the filesystem.
        Self::parse_module_information();

        new_module
    }

    fn parse_module_information() {

    }

}

pub struct CharacterData {
    pub character_name: String,
    pub aliases: Vec<String>,
    pub num_costumes: u16,
}

