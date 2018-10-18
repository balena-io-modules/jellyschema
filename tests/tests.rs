use balena_configuration_dsl::dsl::validation;
use balena_configuration_dsl::ui_configuration::generator::Generator;

#[test]
fn empty_schema() -> Result<(), validation::Error> {
    let (json_schema, ui_object) = Generator::with(input_schema())?.generate();

    assert_eq!(ui_object, expected_ui_object());
    assert_eq!(json_schema, expected_json_schema());
    Ok(())
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
