
use balena_configuration_dsl::Generator;
use balena_configuration_dsl::Compiler;


#[test]
fn support_version() {

    let input_schema = include_str!("data/001-version/input-schema.yml");
    let expected_json_schema = include_str!("data/001-version/output-json-schema.json");
    let expected_ui_object = include_str!("data/001-version/output-uiobject.json");

    let compiler = Compiler::new(input_schema);
    let compiled_schema = compiler.compile();
    let generator = Generator::new(compiled_schema);

    let (json_schema, ui_object) = generator.generate();

    assert_eq!(json_schema, expected_json_schema);
    assert_eq!(ui_object, expected_ui_object);
}