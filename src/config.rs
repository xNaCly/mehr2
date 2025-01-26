use std::collections::HashMap;

use mlua::UserData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ScratchPackage {
    pub identifier: String,
    /// execute scratch building if any of the members is found as a executable on the system
    pub executes_for: Option<Vec<String>>,
    /// the package requires all members to exist for it to build
    pub needs: Option<Vec<String>>,
    /// command to update said package
    pub update: Option<String>,
    pub script: Option<String>,
    /// git url to use for cloning
    pub git: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Packages {
    /// Basic representation for system and specific package mangers
    Packages(Vec<String>),
    ScratchPackages(Vec<ScratchPackage>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub packages: HashMap<String, Packages>,
}

impl UserData for Config {}
