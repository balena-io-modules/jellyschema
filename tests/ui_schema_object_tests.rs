use balena_configuration_dsl::dsl_schema::compiler::Compiler;
use balena_configuration_dsl::ui_config::Generator;

#[test]
fn empty_schema() {
    let compiler = Compiler::new(input_schema());
    let compiled_schema = compiler.compile().unwrap();
    let generator = Generator::new(compiled_schema);

    let (json_schema, ui_object) = generator.generate();

    assert_eq!(ui_object, expected_ui_object());
    assert_eq!(json_schema, expected_json_schema());
}

fn input_schema() -> serde_yaml::Value {
    serde_yaml::from_str(include_str!("data/001-empty-schema/input-schema.yml")).unwrap()
}

fn expected_json_schema() -> serde_json::Value {
    serde_json::from_str(include_str!("data/001-empty-schema/output-json-schema.json")).unwrap()
}

fn expected_ui_object() -> serde_json::Value {
    serde_json::from_str(include_str!("data/001-empty-schema/output-uiobject.json")).unwrap()
}
