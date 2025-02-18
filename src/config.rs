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
    /// packages that can be installed by their names
    Packages(Vec<String>),
    /// packages that have to be made from scratch via commands
    ScratchPackages(Vec<ScratchPackage>),
}

#[derive(Debug, Deserialize, Serialize)]
/// the MEHR2 struct in the mehr2.lua file
pub struct Config {
    pub packages: HashMap<String, Packages>,
}

impl UserData for Config {}
