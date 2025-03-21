use std::process::Command;

use super::PackageManager;

#[derive(Debug)]
pub struct Npm;

impl PackageManager for Npm {
    fn upgrade(&self, packages: &[super::Package]) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("npm")
            .arg("-g")
            .arg("update")
            .args(packages)
            .status()
            .map(|_| {})?)
    }

    fn install(&self, packages: &[super::Package]) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("npm")
            .arg("-g")
            .arg("install")
            .args(packages)
            .status()
            .map(|_| {})?)
    }

    fn update(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
