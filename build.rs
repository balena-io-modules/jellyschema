use std::{
    collections::HashMap,
    env,
    fs::{read_dir, DirEntry, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use strfmt::strfmt;

// TODO This is a mess we should cleanup somehow

fn generate_validator_data_tests() -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR")?;
    let destination = Path::new(&out_dir).join("validator_data_tests.rs");
    let mut test_file = File::create(&destination)?;
    start_module(&mut test_file, "validator_data")?;
    generate_validator_data_tests_module(
        &mut test_file,
        &PathBuf::from_str("./tests/data/validator/data").unwrap(),
    )?;
    end_module(&mut test_file)?;
    Ok(())
}

fn generate_validator_data_tests_module(mut test_file: &mut File, dir: &PathBuf) -> Result<(), Error> {
    let module_name = normalize_file_stem(dir)?;
    start_module(&mut test_file, &module_name)?;

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path().canonicalize()?;

        if path.is_dir() {
            generate_validator_data_tests_module(test_file, &path)?;
        } else {
            match path.extension() {
                Some(ext) if ext == "yaml" => write_validator_data_test(test_file, &path)?,
                _ => {}
            };
        }
    }

    end_module(&mut test_file)?;
    Ok(())
}

fn generate_validator_error_tests() -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR")?;
    let destination = Path::new(&out_dir).join("validator_error_tests.rs");
    let mut test_file = File::create(&destination)?;
    start_module(&mut test_file, "validator_errors")?;
    generate_validator_error_tests_module(
        &mut test_file,
        &PathBuf::from_str("./tests/data/validator/errors").unwrap(),
    )?;
    end_module(&mut test_file)?;
    Ok(())
}

fn generate_validator_error_tests_module(mut test_file: &mut File, dir: &PathBuf) -> Result<(), Error> {
    let module_name = normalize_file_stem(dir)?;
    start_module(&mut test_file, &module_name)?;

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path().canonicalize()?;

        if path.is_dir() {
            generate_validator_error_tests_module(test_file, &path)?;
        } else {
            match path.extension() {
                Some(ext) if ext == "yaml" => write_validator_error_test(test_file, &path)?,
                _ => {}
            };
        }
    }

    end_module(&mut test_file)?;
    Ok(())
}

fn write_validator_data_test(test_file: &mut File, path: &PathBuf) -> Result<(), Error> {
    let name = format!("{}", normalize_file_stem(&path)?);

    write!(
        test_file,
        include_str!("./tests/validator_data_test_template"),
        name = name,
        path = path.display()
    )?;
    Ok(())
}

fn write_validator_error_test(test_file: &mut File, path: &PathBuf) -> Result<(), Error> {
    let name = format!("{}", normalize_file_stem(&path)?);

    write!(
        test_file,
        include_str!("./tests/validator_error_test_template"),
        name = name,
        path = path.display()
    )?;
    Ok(())
}

fn generate_output_tests() -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR")?;
    let destination = Path::new(&out_dir).join("output_tests.rs");
    let mut test_file = File::create(&destination)?;

    let test_data_directories = read_dir("./tests/data/output")?;

    start_module(&mut test_file, "output_generator")?;

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
                write_test(
                    &mut test_file,
                    &data_directory,
                    "./tests/output_invalid_data_test_template",
                )?;
            } else {
                write_test(
                    &mut test_file,
                    &data_directory,
                    "./tests/output_valid_data_test_template",
                )?;
            }
        }

        end_module(&mut test_file)?;
    }

    end_module(&mut test_file)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    generate_validator_data_tests()?;
    generate_validator_error_tests()?;
    generate_output_tests()?;
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
#[allow(unused_imports)]
mod {name} {{
    use jellyschema::schema::Schema;
    use jellyschema::error::Error;
    use jellyschema::output::generate_json_ui_schema;
    use pretty_assertions::assert_eq;
    use serde_json;
    use serde_yaml;
    use std::str::FromStr;

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

fn normalize_file_stem(path: &PathBuf) -> Result<String, Error> {
    let result = path
        .file_stem()
        .ok_or(Error {
            message: "cannot convert os string into string".to_string(),
        })?
        .to_string_lossy()
        .replace("-", "_");
    Ok(result.to_string())
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
