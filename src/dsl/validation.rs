#[derive(Debug)]
pub struct Error {
    message: String
}

impl Error {
    pub fn invalid_version(version: u64) -> Self{
        Error {
            message: format!("Invalid version specified: {}", version)
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(source: serde_yaml::Error) -> Self {
        Error {
            message: source.to_string()
        }
    }
}