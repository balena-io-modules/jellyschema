use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::io::Read;
use std::collections::HashMap;

use strfmt::strfmt;

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
            let data_directory = data_directory?;
            let files = read_dir(&data_directory.path())?;
            if files
                .into_iter()
                .map(|entry| entry.unwrap().file_name().into_string())
                .any(|file_name| file_name.unwrap() == "output-error")
            {
                write_test(&mut test_file, &data_directory, "./tests/invalid_data_test_template")?;
            } else {
                write_test(&mut test_file, &data_directory, "./tests/valid_data_test_template")?;
            }
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
    use balena_cdsl::dsl::schema::compiler::CompilationError;
    use balena_cdsl::output::generator::Generator;
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

fn write_test(test_file: &mut File, directory: &DirEntry, template_path: &str) -> Result<(), Error> {
    let directory = directory.path().canonicalize()?;
    let path = directory.display();
    let name = format!("{}", file_name(&directory)?);

    let mut template_file = File::open(template_path)?;
    let mut template = String::new();
    template_file.read_to_string(&mut template)?;

    let mut vars = HashMap::new();
    vars.insert("name".to_string(), name);
    vars.insert("path".to_string(), path.to_string());

    let rendered_contents = strfmt(&template, &vars).expect("Cannot format template");

    test_file.write_all(rendered_contents.as_bytes())?;

    Ok(())
}
