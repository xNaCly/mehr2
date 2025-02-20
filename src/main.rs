use std::{
    env::{self},
    process::exit,
};

use config::Config;
use lock::Lock;
use log::{error, info, trace, warn, LevelFilter};
use managers::process_packages;

/// config contains the logic for deserializing mehr2.lua
mod config;
/// lock abstracts working with mehr2_lock.json files
mod lock;
/// managers contains the PackageManager trait and its implementations
mod managers;
/// path deals with looking up executables and file paths
mod path;

fn main() {
    colog::basic_builder()
        .filter(None, LevelFilter::max())
        .init();
    let config_dir_path = path::config()
        .map(|e| std::path::absolute(e).unwrap())
        .unwrap();
    let configuration_path = config_dir_path.join("mehr2.lua");
    trace!("using configuration file: {:?}", configuration_path);
    let lock_path = config_dir_path.join("lock.mehr2");
    trace!("using lock file: {:?}", lock_path);
    let lua_ctx = mlua::Lua::new();
    let config = match Config::from_path_buf(&lua_ctx, configuration_path) {
        Ok(conf) => conf,
        Err(err) => {
            error!("{err}");
            exit(1);
        }
    };

    let lock: Option<Lock> = (&lock_path).try_into().inspect_err(|e| warn!("{e}")).ok();
    let command = match env::args().nth(1) {
        Some(command) => command,
        None => {
            info!("Got no command, defaulting to sync");
            "sync".to_string()
        }
    };
    match command.as_str() {
        "sync" => {
            if let Err(err) = process_packages(config) {
                error!("{err}");
                exit(1);
            } else {
                lock.inspect(|l| {
                    if let Err(err) = l.dump(&lock_path) {
                        warn!("{err}")
                    }
                });
            }
        }
        "update" => todo!("update"),
        c @ _ => {
            error!("Unkown command {c}, use 'sync' or 'update'");
            exit(1);
        }
    }
}
