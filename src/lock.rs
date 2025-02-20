use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::PathBuf};

use crate::config::{
    Config,
    Packages::{Packages, ScratchPackages},
};

/// Lock holds a map of installed packages, by their package manager, stored in lock.mehr2
#[derive(Serialize, Deserialize)]
pub struct Lock {
    packages: HashMap<String, Vec<String>>,
}

impl Lock {
    /// dumps self to path
    pub fn dump(&self, path: &PathBuf) -> Result<(), String> {
        let file = File::options()
            .write(true)
            .open(path)
            .map_err(|err| format!("failed to open lockfile: {err}"))?;
        Ok(serde_lexpr::to_writer(file, self)
            .map_err(|err| format!("failed to serialize into lockfile: {err}"))
            .map(|_| ())?)
    }

    pub fn diff(&self, config: &Config) -> HashMap<String, Vec<String>> {
        let other: Lock = config.into();
        todo!()
    }
}

impl From<&Config> for Lock {
    fn from(value: &Config) -> Self {
        let mut lock = Self {
            packages: HashMap::new(),
        };
        value.packages.iter().for_each(|(key, value)| {
            let package_names: Vec<String> = match value {
                Packages(packages) => packages.clone(),
                ScratchPackages(packages) => {
                    packages.into_iter().map(|p| p.identifier.clone()).collect()
                }
            };
            lock.packages.insert(key.clone(), package_names);
        });
        lock
    }
}

impl TryFrom<&PathBuf> for Lock {
    type Error = String;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let file = File::open(value).map_err(|err| format!("failed to open lock file: {err}"))?;
        serde_lexpr::from_reader(file)
            .map_err(|err| format!("failed to deserialize lock file: {err}"))?
    }
}
