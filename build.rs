use std::fs::File;
use std::io::Write;
use std::fs::read_dir;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("tests.rs");
    let mut test_file = std::fs::File::create(&destination).unwrap();


    let test_data_directories = read_dir("./tests/data/").unwrap();

    write!(test_file,
    r#"
use balena_configuration_dsl::dsl::validation;
use balena_configuration_dsl::ui_configuration::generator::Generator;
use std::fs::File;
use std::io::Read;
use std::path::Path;
"#).unwrap();

    for directory in test_data_directories {
        let directory = directory.unwrap().path().canonicalize().unwrap();
        let path = directory.display();
        let name = format!("case_{}", directory.file_name().unwrap().to_string_lossy());

        write!(
            test_file,
            r###"
#[test]
fn {name}() -> Result<(), validation::Error> {{
    let input_schema = serde_yaml::from_str(include_str!("{path}/input-schema.yml")).unwrap();
    let (json_schema, ui_object) = Generator::with(input_schema)?.generate();

    let expected_json_schema : serde_json::Value = serde_json::from_str(include_str!("{path}/output-json-schema.json")).unwrap();
    let expected_ui_object : serde_json::Value  = serde_json::from_str(include_str!("{path}/output-uiobject.json")).unwrap();
    assert_eq!(ui_object, expected_ui_object);
    assert_eq!(json_schema, expected_json_schema);
    Ok(())
}}"###,
            name = name, path = path
        ).unwrap();

    }

}