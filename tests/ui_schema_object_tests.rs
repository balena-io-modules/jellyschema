
use balena_configuration_dsl::Generator;
use balena_configuration_dsl::Compiler;
use serde_json::json;

#[test]
fn support_version() -> Result<(), (serde_yaml::Error)>{

    let input_schema = serde_yaml::from_str(include_str!("data/001-version/input-schema.yml"))?;
    let expected_json_schema = json!(include_str!("data/001-version/output-json-schema.json"));
    let expected_ui_object = json!(include_str!("data/001-version/output-uiobject.json"));

    let compiler = Compiler::new(input_schema);
    let compiled_schema = compiler.compile();
    let generator = Generator::new(compiled_schema);

    let (json_schema, ui_object) = generator.generate();

    // FIXME: better comparator
    assert_eq!(json_schema, expected_json_schema);
    assert_eq!(ui_object, expected_ui_object);
    Ok(())
}