use budget_lib::Cache;
use budget_lib::Ledger;
use budget_lib::Budget;
use decimal::d128;
use std::path::PathBuf;
use serde_yaml;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

pub struct PersistentBudget {
    directory: PathBuf,
    budget: Budget<d128>,
}

impl PersistentBudget {
    pub fn new<P: Into<PathBuf>>(directory: P) -> Self {
        PersistentBudget{directory: directory.into(), budget: Budget::default()}
    }
    pub fn from_directory<P: Into<PathBuf>>(directory: P) -> Result<Self, SerialiseError> {
        let mut b = PersistentBudget::new(directory);
        b.load()?;
        Ok(b)
    }

    pub fn load(&mut self) -> Result<(),SerialiseError> {
        let transactions: Ledger = from_file(self.ledger_path())?;
        let cache: Cache<d128> = from_file(self.cache_path())?;
        let b = Budget::new(transactions, cache);
        self.budget = b;
        Ok(())
    }

    pub fn save(&self) -> Result<(), SerialiseError> {
        to_file(self.budget.ledger(), self.ledger_path())?;
        to_file(self.budget.cache(), self.cache_path())?;
        Ok(())
    }

    pub fn ledger_path(&self) -> PathBuf {
        self.directory.join("ledger")
    }

    pub fn cache_path(&self) -> PathBuf {
        self.directory.join("cache")
    }
}

impl std::ops::Deref for PersistentBudget {
    type Target = Budget<d128>;

    fn deref(&self) -> &Self::Target {
        &self.budget
    }
}

impl std::ops::DerefMut for PersistentBudget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.budget
    }
}




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

impl Error for SerialiseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SerialiseError::Io(e) => Some(e),
            SerialiseError::Parse(e) => Some(e),
        }
    }
}

pub fn to_file<T: Serialize, P: AsRef<Path>>(t: &T, path: P) -> Result<(), SerialiseError> {
    match File::create(path) {
        Err(x) => Err(SerialiseError::Io(x)),
        Ok(file) => match serde_yaml::to_writer(file, t) {
            Err(x) => Err(SerialiseError::Parse(x)),
            Ok(_) => Ok(()),
        },
    }
}

pub fn from_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, SerialiseError> {
    match File::open(path) {
        Err(x) => Err(SerialiseError::Io(x)),
        Ok(file) => match serde_yaml::from_reader(file) {
            Err(x) => Err(SerialiseError::Parse(x)),
            Ok(x) => Ok(x),
        },
    }
}
