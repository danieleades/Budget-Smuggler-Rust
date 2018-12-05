use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;
use std::error::Error;

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

pub fn serialise_to_file<T: Serialize, P: AsRef<Path>>(t: &T, path: P) -> Result<(), SerialiseError> {
    match File::create(path) {
        Err(x) => return Err(SerialiseError::Io(x)),
        Ok(file) => match serde_yaml::to_writer(file, t) {
            Err(x) => return Err(SerialiseError::Parse(x)),
            Ok(_) => Ok(()),
        },
    }
}

pub fn deserialise_from_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, SerialiseError> {
    match File::open(path) {
        Err(x) => return Err(SerialiseError::Io(x)),
        Ok(file) => match serde_yaml::from_reader(file) {
            Err(x) => return Err(SerialiseError::Parse(x)),
            Ok(x) => Ok(x),
        },
    }
}
