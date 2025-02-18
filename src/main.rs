use std::{
    env::{self, args},
    fs,
    path::PathBuf,
    process::exit,
};

use anyhow::Result;
use log::{error, info, trace, LevelFilter};
use managers::process_packages;
use mlua::LuaSerdeExt;

/// config contains the logic for deserializing mehr2.lua
mod config;
/// lock abstracts working with mehr2_lock.json files
mod lock;
/// managers contains the PackageManager trait and its implementations
mod managers;
/// path deals with looking up executables and file paths
mod path;

fn load_configuration(lua: &mlua::Lua, path: PathBuf) -> Result<config::Config, String> {
    let path_clone = path.clone();
    let path_as_str = path_clone.to_str().unwrap_or_else(|| "invalid utf8");
    trace!("loading configuration");
    let config_as_str = fs::read_to_string(path).map_err(|err| {
        format!(
            "Failed to read configuration file '{}': {}",
            path_as_str, err
        )
    })?;

    lua.load(config_as_str)
        .set_name(path_as_str.to_string())
        .exec()
        .map_err(|err| format!("{}: {}", path_as_str, err))?;

    let raw_conf = lua
        .globals()
        .get::<mlua::Value>("MEHR2")
        .map_err(|err| format!("{}: {}", path_as_str, err))?;

    if raw_conf.is_nil() {
        return Err(format!(
            "{}: MEHR2 table is missing from configuration",
            path_as_str
        ));
    }

    lua.from_value(raw_conf)
        .map_err(|err| format!("{}: {}", path_as_str, err))
}

fn main() {
    colog::basic_builder()
        .filter(None, LevelFilter::max())
        .init();
    let config_dir_path = path::config()
        .map(|e| std::path::absolute(e).unwrap())
        .unwrap();
    let configuration_path = config_dir_path.join("mehr2.lua");
    trace!("using configuration file: {:?}", configuration_path);
    let lock_path = config_dir_path.join("mehr2_lock.json");
    trace!("using lock file: {:?}", lock_path);
    let lua_ctx = mlua::Lua::new();
    let config = match load_configuration(&lua_ctx, configuration_path) {
        Ok(conf) => conf,
        Err(err) => {
            error!("{err}");
            exit(1);
        }
    };

    if let Some(command) = env::args().nth(1) {
        match command.as_str() {
            "sync" => {
                if let Err(err) = process_packages(config) {
                    error!("{err}");
                    exit(1);
                }
            }
            "update" => todo!("update"),
            c @ _ => {
                error!("Unkown command {c}, use 'sync' or 'update'");
                exit(1);
            }
        }
    } else {
        info!("Got no command, defaulting to sync");
        if let Err(err) = process_packages(config) {
            error!("{err}");
            exit(1);
        }
    }
}
