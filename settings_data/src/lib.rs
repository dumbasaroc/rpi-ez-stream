mod datatypes;
use datatypes::GTKSettingsDatatypes as SD;
use std::path::Path;

/* ==== KEY DEFINITIONS ==== */

pub const FIRST_TIME_USE_KEY: &str = "first-time-opening-app";
pub const LAST_OPENED_VER_MAJOR_KEY: &str = "last-opened-version-major";
pub const LAST_OPENED_VER_MINOR_KEY: &str = "last-opened-version-minor";
pub const LAST_OPENED_VER_PATCH_KEY: &str = "last-opened-version-patch";

pub const P1_NAME_SETTING_KEY: &str = "p1-name";
pub const P2_NAME_SETTING_KEY: &str = "p2-name";
pub const P1_SCORE_SETTING_KEY: &str = "p1-score";
pub const P2_SCORE_SETTING_KEY: &str = "p2-score";


const SETTINGS: [datatypes::GTKSettingsDatatypes; 8] = [
    SD::Bool {
        key: FIRST_TIME_USE_KEY,
        val: true,
        description: "Whether or not this is the first time entering EZ-Stream."
    },

    SD::U32 {
        key: LAST_OPENED_VER_MAJOR_KEY,
        val: 0,
        description: "The most recently opened version of the application. Used for first-time experience (TBC)"
    },

    SD::U32 {
        key: LAST_OPENED_VER_MINOR_KEY,
        val: 0,
        description: "The most recently opened version of the application. Used for first-time experience (TBC)"
    },

    SD::U32 {
        key: LAST_OPENED_VER_PATCH_KEY,
        val: 0,
        description: "The most recently opened version of the application. Used for first-time experience (TBC)"
    },

    SD::U32 {
        key: P1_SCORE_SETTING_KEY,
        val: 0,
        description: "The last score typed into the P1 score box"
    },

    SD::U32 {
        key: P2_SCORE_SETTING_KEY,
        val: 0,
        description: "The last score typed into the P2 score box"
    },

    SD::String {
        key: P1_NAME_SETTING_KEY,
        val: "",
        description: "The last inputted name for P1"
    },

    SD::String {
        key: P2_NAME_SETTING_KEY,
        val: "",
        description: "The last inputted name for P2"
    },
];

pub fn export_settings_to_xml<P>(path: P) -> Result<(), String> where P: AsRef<Path> {
    use std::io::Write;

    let mut outfile = match std::fs::File::create(path) {
        Ok(f) => f,
        Err(e) => { return Err(format!("{}", e)); }
    };

    writeln!(outfile, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>").unwrap();
    writeln!(outfile, "<?xml-model href=\"gschema.dtd\" type=\"application/xml-dtd\"?>").unwrap();
    writeln!(outfile, "<schemalist>").unwrap();
    writeln!(outfile, "<schema id=\"edu.rpi.smashclub.ezstream\" path=\"/edu/rpi/smashclub/ezstream/\">").unwrap();

    for setting in SETTINGS {
        write!(outfile, "{}", setting).unwrap();
    }

    writeln!(outfile, "</schema>").unwrap();
    writeln!(outfile, "</schemalist>").unwrap();

    Ok(())
}

