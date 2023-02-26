mod config;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use clap::{Parser, Subcommand};
use config::CosmwasmChainConfig;
use serde_json::{from_value, from_str, to_value, to_string_pretty};
use json_patch::merge;

use crate::config::{default_config};

#[derive(Parser)]
#[command(name = "ephemeris", author = "mintthemoon <mint@mintthemoon.xyz>", version = "0.1.0")]
#[command(about = "Configure your node environment")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// configure app.toml
    ConfigApp {
        /// chain id
        #[arg(short, long)]
        chain: String,
        /// output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// parameter overrides json
        #[arg(long)]
        custom: Option<String>,
    },
    /// configure config.toml
    ConfigTendermint {
        /// chain id
        #[arg(short, long)]
        chain: String,
        /// output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// parameter overrides json
        #[arg(long)]
        custom: Option<String>,
    },
    /// configure genesis.json
    ConfigGenesis {
        /// chain id
        #[arg(short, long)]
        chain: String,
        /// output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// parameter overrides json
        #[arg(long)]
        custom: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::ConfigApp { chain, output, custom }) => {
            let default_cfg = default_config(chain).unwrap();
            let cfg: CosmwasmChainConfig = match custom {
                Some(c) => {
                    let patch = from_str(&format!("{{\"app_config\": {}}}", c)).unwrap();
                    let mut base = to_value(default_cfg).unwrap();
                    merge(&mut base, &patch);
                    from_value(base).unwrap()
                },
                None => default_cfg,
            };
            let path = match output {
                Some(o) => o.join("app.toml"),
                None => PathBuf::new().join("app.toml"),
            };
            cfg.render_app_config(&path).unwrap();
        },
        Some(Commands::ConfigTendermint { chain, output, custom }) => {
            let default_cfg = default_config(chain).unwrap();
            let cfg: CosmwasmChainConfig = match custom {
                Some(c) => {
                    let patch = from_str(&format!("{{\"tendermint_config\": {}}}", c)).unwrap();
                    let mut base = to_value(default_cfg).unwrap();
                    merge(&mut base, &patch);
                    from_value(base).unwrap()
                },
                None => default_cfg,
            };
            let path = match output {
                Some(o) => o.join("config.toml"),
                None => PathBuf::new().join("config.toml"),
            };
            cfg.render_tendermint_config(&path).unwrap();
        },
        Some(Commands::ConfigGenesis { chain, output, custom }) => {
            let cfg = default_config(chain).unwrap();
            let path = match output {
                Some(o) => o.join("genesis.json"),
                None => PathBuf::new().join("genesis.json"),
            };
            let default_genesis = cfg.download_genesis().unwrap();
            let genesis = match custom {
                Some(c) => {
                    let patch = from_str(&c).unwrap();
                    let mut base = from_str(&default_genesis).unwrap();
                    merge(&mut base, &patch);
                    to_string_pretty(&base).unwrap()
                },
                None => default_genesis,
            };
            File::create(path).unwrap().write_all(genesis.as_bytes()).unwrap();
        },
        None => {},
    }
}
