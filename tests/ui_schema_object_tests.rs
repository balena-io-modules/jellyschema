use balena_configuration_dsl::dsl_schema::Compiler;
use balena_configuration_dsl::ui_config::Generator;
use serde_json::json;

#[test]
fn support_version() {
    let compiler = Compiler::new(input_schema());
    let compiled_schema = compiler.compile();
    let generator = Generator::new(compiled_schema);

    let (json_schema, ui_object) = generator.generate();

    assert_eq!(ui_object, output_ui_object());
    assert_eq!(json_schema, output_json_schema());
}

fn input_schema() -> serde_yaml::Value {
    serde_yaml::from_str(include_str!("data/001-version/input-schema.yml")).unwrap()
}

fn output_json_schema() -> serde_json::Value {
    serde_json::from_str(include_str!("data/001-version/output-json-schema.json")).unwrap()
}

fn output_ui_object() -> serde_json::Value {
    serde_json::from_str((include_str!("data/001-version/output-uiobject.json"))).unwrap()
}
