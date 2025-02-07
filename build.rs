use core::str;
use std::process::Command;

fn main() {

    const LOC_OF_SETTINGS_SCHEMA: &str = "data/edu.rpi.smashclub.ezstream.gschema.xml";

    // Rerun build if the settings schema has changed
    println!("cargo::rerun-if-changed={}", LOC_OF_SETTINGS_SCHEMA);

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