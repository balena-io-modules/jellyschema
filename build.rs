use std::fs::read_dir;
use std::io::Write;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("tests.rs");
    let mut test_file = std::fs::File::create(&destination).unwrap();

    let test_data_directories = read_dir("./tests/data/").unwrap();

    write!(
        test_file,
        r#"use balena_configuration_dsl::dsl::validation;
use balena_configuration_dsl::ui_configuration::generator::Generator;
"#
    )
    .unwrap();

    for directory in test_data_directories {
        let directory = directory.unwrap().path().canonicalize().unwrap();
        let path = directory.display();
        let name = format!("case_{}", directory.file_name().unwrap().to_string_lossy());

        // TODO: add differ (treediff)

        write!(
            test_file,
            r###"
#[test]
fn {name}() -> Result<(), validation::Error> {{
    println!();
    println!("starting test {name}");
    let input_schema : serde_yaml::Value = serde_yaml::from_str(
        include_str!("{path}/input-schema.yml")).
        unwrap();
    println!("loaded input schema: {{:?}}", input_schema);
    println!();
    let expected_json_schema : serde_json::Value = serde_json::from_str(
        include_str!("{path}/output-json-schema.json")).
        unwrap();
    println!("loaded expected json schema: {{:?}}", expected_json_schema);
    println!();
    let expected_ui_object : serde_json::Value  = serde_json::from_str(
        include_str!("{path}/output-uiobject.json")).
        unwrap();
    println!("loaded expected ui object: {{:?}}", expected_ui_object);
    println!();

    let (json_schema, ui_object) = Generator::with(input_schema)?.generate();

    println!("comparing json schemas");
    assert_eq!(json_schema, expected_json_schema, "Actual json schema different than expected");
    println!("comparing ui objects");
    assert_eq!(ui_object, expected_ui_object, "Actual ui object different than expected");

    Ok(())
}}
"###,
            name = name,
            path = path
        )
        .unwrap();
    }
}
