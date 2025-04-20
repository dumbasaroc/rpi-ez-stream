use core::str;
use std::process::Command;
use settings_data::export_settings_to_xml;

fn main() {

    glib_build_tools::compile_resources(
        &[ "ui" ],
        "ui/ui_resource.gresource.xml",
        "ui_resource.gresource"
    );

    const LOC_OF_SETTINGS_SCHEMA: &str = "data/edu.rpi.smashclub.ezstream.gschema.xml";
    export_settings_to_xml(LOC_OF_SETTINGS_SCHEMA).unwrap();


    // Call the compilation of the schema, if there are errors, abort the
    // build by panicking
    let compile_schema_cmd = Command::new("glib-compile-schemas")
        .args(["--strict", "./data"])
        .output();

    let compile_schema_output = match compile_schema_cmd {
        Ok(o) => o,
        Err(e) => {
            panic!("Call of `glib-compile-schemas` failed: {}", e);
        }
    };

    if !compile_schema_output.status.success() {
        let compile_schema_output_str = str::from_utf8(&compile_schema_output.stderr).unwrap();
        panic!("Errors while compiling glib schemas: {}", compile_schema_output_str);
    }
}
