use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum SerialiseError {
    Io(io::Error),
    Parse(serde_yaml::Error),
}

impl fmt::Display for SerialiseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SerialiseError::Io(e) => write!(f, "IO error: {}", e),
            SerialiseError::Parse(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl Error for SerialiseError {}

pub fn serialise_to_file<T: Serialize, P: AsRef<Path>>(
    t: &T,
    path: P,
) -> Result<(), SerialiseError> {
    match File::create(path) {
        Err(x) => Err(SerialiseError::Io(x)),
        Ok(file) => match serde_yaml::to_writer(file, t) {
            Err(x) => Err(SerialiseError::Parse(x)),
            Ok(_) => Ok(()),
        },
    }
}

pub fn deserialise_from_file<T: DeserializeOwned, P: AsRef<Path>>(
    path: P,
) -> Result<T, SerialiseError> {
    match File::open(path) {
        Err(x) => Err(SerialiseError::Io(x)),
        Ok(file) => match serde_yaml::from_reader(file) {
            Err(x) => Err(SerialiseError::Parse(x)),
            Ok(x) => Ok(x),
        },
    }
}
