use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR")?;
    let destination = Path::new(&out_dir).join("tests.rs");
    let mut test_file = File::create(&destination)?;

    let test_data_directories = read_dir("./tests/data/")?;

    for category_directory in test_data_directories {
        let category_directory = category_directory?.path().canonicalize()?;
        let data_directories = read_dir(&category_directory)?;

        start_module(&mut test_file, &file_name(&category_directory)?)?;

        for data_directory in data_directories {
            write_test(&mut test_file, &data_directory?)?;
        }

        end_module(&mut test_file)?;
    }
    Ok(())
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<env::VarError> for Error {
    fn from(error: env::VarError) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

fn start_module(test_file: &mut File, name: &str) -> Result<(), Error> {
    write!(
        test_file,
        r#"
mod {name} {{
    use balena_configuration_dsl::dsl::compiler::CompilationError;
    use balena_configuration_dsl::output::generator::Generator;
    use pretty_assertions::assert_eq;

        "#,
        name = name
    )?;
    Ok(())
}

fn end_module(test_file: &mut File) -> Result<(), Error> {
    write!(
        test_file,
        r#"
}}

        "#
    )?;
    Ok(())
}
fn file_name(directory: &PathBuf) -> Result<String, Error> {
    let result = directory
        .file_name()
        .ok_or(Error {
            message: "cannot convert os string into string".to_string(),
        })?
        .to_string_lossy();
    Ok(result.to_string())
}

fn write_test(test_file: &mut File, directory: &DirEntry) -> Result<(), Error> {
    let directory = directory.path().canonicalize()?;
    let path = directory.display();
    let name = format!("{}", file_name(&directory)?);

    write!(
        test_file,
        include_str!("./tests/test_template"),
        name = name,
        path = path
    )?;
    Ok(())
}
