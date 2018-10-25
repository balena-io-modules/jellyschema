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
use pretty_assertions;
use pretty_assertions::assert_eq;
"#
    )
    .unwrap();

    for directory in test_data_directories {
        let directory = directory.unwrap().path().canonicalize().unwrap();
        let path = directory.display();
        let name = format!("case_{}", directory.file_name().unwrap().to_string_lossy());

        write!(
            test_file,
            include_str!("./tests/test_template"),
            name = name,
            path = path
        )
        .unwrap();
    }
}
