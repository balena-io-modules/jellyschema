use balena_configuration_dsl::dsl_schema::compiler::Compiler;
use balena_configuration_dsl::dsl_schema::Schema;
use balena_configuration_dsl::ui_config::Generator;
use balena_configuration_dsl::ui_config::JsonSchema;
use balena_configuration_dsl::ui_config::UIObject;

#[test]
fn empty_schema() {
    let compiler = Compiler::new(input_schema());
    let compiled_schema = compiler.compile().unwrap();
    let generator = Generator::new(compiled_schema);

    let (json_schema, ui_object) = generator.generate();

    assert_eq!(ui_object, expected_ui_object());
    assert_eq!(json_schema, expected_json_schema());
}

fn input_schema() -> Schema {
    serde_yaml::from_str(include_str!("data/001-empty-schema/input-schema.yml")).unwrap()
}

fn expected_json_schema() -> JsonSchema {
    serde_json::from_str(include_str!("data/001-empty-schema/output-json-schema.json")).unwrap()
}

fn expected_ui_object() -> UIObject {
    serde_json::from_str(include_str!("data/001-empty-schema/output-uiobject.json")).unwrap()
}
