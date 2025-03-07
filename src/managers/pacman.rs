use std::process::Command;

use super::PackageManager;

#[derive(Debug)]
pub struct Pacman;

impl PackageManager for Pacman {
    fn upgrade(&self, packages: &[super::Package]) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .args(packages)
            .status()
            .map(|_| {})?)
    }

    fn install(&self, packages: &[super::Package]) -> anyhow::Result<()> {
        Pacman.upgrade(packages)
    }

    fn update(&self) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("pacman")
            .arg("-Sy")
            .status()
            .map(|_| {})?)
    }
}
