use anyhow::Result;

mod npm;
mod pacman;

use log::warn;

use crate::{
    config::{self, Packages},
    lock,
};

pub fn process_packages(config: config::Config) -> Result<()> {
    if let Some(default_packages) = config.packages.get("default") {
        if let Packages::Packages(packages) = default_packages {
            if let Some(system_manager) = default() {
                system_manager.update()?;
                system_manager.install(packages)?;
            } else {
                warn!("could not determine a default package manager")
            }
        }
    }

    // TODO: match specific package managers here

    Ok(())
}

type Package = String;

pub trait PackageManager {
    fn upgrade(&self, packages: &[Package]) -> Result<()>;
    fn install(&self, packages: &[Package]) -> Result<()>;
    /// update syncs the current package manager to its repos
    fn update(&self) -> Result<()>;
}

/// default returns the default package manager for the given system
pub fn default() -> Option<Box<dyn PackageManager>> {
    None
}

pub fn exists(name: &str) -> Option<Box<dyn PackageManager>> {
    Some(match name {
        "npm" => Box::new(npm::Npm {}),
        "pacman" => Box::new(pacman::Pacman {}),
        "cargo" => todo!(),
        "go" => todo!(),
        _ => return None,
    })
}
