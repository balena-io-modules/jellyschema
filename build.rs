use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("tests.rs");
    let mut test_file = File::create(&destination).unwrap();

    write_header(&mut test_file);

    let test_data_directories = read_dir("./tests/data/").unwrap();

    for directory in test_data_directories {
        write_test(&mut test_file, &directory.unwrap());
    }
}

fn write_test(test_file: &mut File, directory: &DirEntry) {
    let directory = directory.path().canonicalize().unwrap();
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

fn write_header(test_file: &mut File) {
    write!(
        test_file,
        r#"
use balena_configuration_dsl::dsl::compiler::CompilationError;
use balena_configuration_dsl::ui_configuration::generator::Generator;
use pretty_assertions::assert_eq;
"#
    )
    .unwrap();
}
