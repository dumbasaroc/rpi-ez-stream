use std::path::PathBuf;
use serde::Deserialize;

pub struct ModuleHandler {
    pub current_module_name: String,
    pub base_directory_path: PathBuf,
    pub characters: Vec<CharacterData>,
    pub default_character: CharacterData,
}

#[derive(Deserialize, Clone, PartialEq, Eq)]
pub struct CharacterData {
    pub display_name: String,
    pub path_in_folder: String,
    pub num_costumes: u32,
    pub aliases: Vec<String>
}

#[derive(Deserialize)]
struct ModuleData {
    pub module_name: String,
    pub default_character: String,
    pub character_data: Vec<CharacterData>
}

impl ModuleHandler {

    pub fn new<P>(path: P) -> anyhow::Result<Self> where P: AsRef<std::path::Path> {
        let mut new_module = Self {
            current_module_name: "Something".to_string(),
            base_directory_path: PathBuf::from(path.as_ref()),
            characters: vec![],
            default_character: CharacterData {
                display_name: "Nil".to_string(),
                path_in_folder: "NIL".to_string(),
                num_costumes: 0,
                aliases: vec![]
            }
        };

        new_module.parse_module_information()?;

        Ok(new_module)
    }

    fn parse_module_information(&mut self) -> anyhow::Result<()> {

        let path = self.base_directory_path.clone();
        let module_info_path = path.join("module_info.json");
        let module_info_json = match std::fs::read_to_string(module_info_path) {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!("Error opening module_info.json: {}", e));
            }
        };

        let module_info = match serde_json::from_str::<ModuleData>(&module_info_json) {
            Ok(d) => d,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to parse module_info.json: {}", e));
            }
        };

        self.current_module_name = module_info.module_name;

        for character in &module_info.character_data {
            
            // @TODO Add some verification code here that ensures the
            // data in the JSON is correct as according to the filesystem
            // ...at current, it just assumes the JSON is infallible.

            self.characters.push(character.clone());
            if character.display_name == module_info.default_character {
                self.default_character = character.clone();
            }
        }

        if self.default_character.display_name == "Nil" {
            return Err(anyhow::anyhow!("The module_info.json provided has an invalid default character!"));
        }

        Ok(())
    }

}
