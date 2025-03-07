use std::process::Command;

use super::PackageManager;

#[derive(Debug)]
pub struct Apt;

impl PackageManager for Apt {
    fn upgrade(&self, packages: &[super::Package]) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("apt")
            .arg("upgrade")
            .args(packages)
            .status()
            .map(|_| {})?)
    }

    fn install(&self, packages: &[super::Package]) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("apt")
            .arg("install")
            .args(packages)
            .status()
            .map(|_| {})?)
    }

    fn update(&self) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("apt")
            .arg("update")
            .status()
            .map(|_| {})?)
    }
}
