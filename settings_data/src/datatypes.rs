pub enum GTKSettingsDatatypes {
    Bool { key: &'static str, val: bool, description: &'static str },
    U32 { key: &'static str, val: u32, description: &'static str },
    String {key: &'static str, val: &'static str, description: &'static str }
}

impl std::fmt::Display for GTKSettingsDatatypes {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool { key, val, description } => {
                writeln!(f, "<key name=\"{}\" type=\"{}\">", key, "b").unwrap();
                writeln!(f, "    <default>{}</default>", val).unwrap();
                writeln!(f, "    <description>{}</description>", description).unwrap();
                writeln!(f, "</key>")
            },
            Self::U32 { key, val, description } => {
                writeln!(f, "<key name=\"{}\" type=\"{}\">", key, "u").unwrap();
                writeln!(f, "    <default>{}</default>", val).unwrap();
                writeln!(f, "    <description>{}</description>", description).unwrap();
                writeln!(f, "</key>")
            },
            Self::String { key, val, description } => {
                writeln!(f, "<key name=\"{}\" type=\"{}\">", key, "s").unwrap();
                writeln!(f, "    <default>\"{}\"</default>", val).unwrap();
                writeln!(f, "    <description>{}</description>", description).unwrap();
                writeln!(f, "</key>")
            }
        }
    }
}

