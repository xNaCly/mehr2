use std::{
    env::{self},
    process::exit,
};

use clap::Parser;

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

#[derive(Debug, PartialEq, Clone, clap::ValueEnum)]
enum Command {
    /// sync local state to remove state
    Sync,
    /// sync repositories to remote state, update locally installed packages to latest available
    /// versions
    Update,
    /// restore all changes made by mehr2
    Clean,
}

/// Operating system-independent package managment abstraction
#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(value_enum)]
    cmd: Command,
}

fn main() {
    let args = Args::parse();
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
    let config = match Config::from_path_buf(&lua_ctx, configuration_path) {
        Ok(conf) => conf,
        Err(err) => {
            error!("{err}");
            exit(1);
        }
    };

    let lock: Option<Lock> = (&lock_path).try_into().inspect_err(|e| warn!("{e}")).ok();
    match args.cmd {
        Command::Sync => match process_packages(config, lock.unwrap_or_default()) {
            Ok(lock) => {
                if let Err(err) = lock.dump(&lock_path) {
                    warn!("{err}")
                }
            }
            Err(err) => warn!("{err}"),
        },
        Command::Update => todo!("update"),
        Command::Clean => todo!("clean"),
    }
}
