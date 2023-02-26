mod config;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use config::CosmwasmChainConfig;
use serde_json::{from_value, from_str, to_value, to_string_pretty};
use json_patch::merge;
use log::{info, error};
use gethostname::gethostname;

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
        /// node moniker
        #[arg(long)]
        moniker: Option<String>,
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

fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    File::create(path)?.write_all(&content.as_bytes())?;
    info!("wrote {}", path.to_string_lossy());
    Ok(())
}

fn config_app(chain: &str, output: &Option<PathBuf>, custom: &Option<String>) -> Result<()> {
    let default_cfg = default_config(chain).ok_or(anyhow!("chain not supported: {}", chain))?;
    let cfg: CosmwasmChainConfig = match custom {
        Some(c) => {
            let patch = from_str(&format!("{{\"app_config\": {}}}", c))?;
            let mut base = to_value(default_cfg)?;
            merge(&mut base, &patch);
            info!("customized app config");
            from_value(base)?
        },
        None => default_cfg,
    };
    let path = match output {
        Some(o) => o.join("app.toml"),
        None => PathBuf::new().join("app.toml"),
    };
    write_file(&path, &cfg.get_app_config()?)?;
    Ok(())
}

fn config_tendermint(chain: &str, output: &Option<PathBuf>, custom: &Option<String>, moniker: &Option<String>) -> Result<()> {
    let default_cfg = default_config(chain).ok_or(anyhow!("chain not supported: {}", chain))?;
    let mut cfg: CosmwasmChainConfig = match custom {
        Some(c) => {
            let patch = from_str(&format!("{{\"tendermint_config\": {}}}", c))?;
            let mut base = to_value(default_cfg)?;
            merge(&mut base, &patch);
            info!("customized tendermint config");
            from_value(base)?
        },
        None => default_cfg,
    };
    match moniker {
        Some(m) => { cfg.tendermint_config.moniker = m.clone(); },
        None => { cfg.tendermint_config.moniker = gethostname().into_string().unwrap_or("node".to_string()); }
    }
    let path = match output {
        Some(o) => o.join("config.toml"),
        None => PathBuf::new().join("config.toml"),
    };
    write_file(&path, &cfg.get_tendermint_config()?)?;
    Ok(())
}

fn config_genesis(chain: &str, output: &Option<PathBuf>, custom: &Option<String>) -> Result<()> {
    let cfg = default_config(chain).ok_or(anyhow!("chain not supported: {}", chain))?;
    let path = match output {
        Some(o) => o.join("genesis.json"),
        None => PathBuf::new().join("genesis.json"),
    };
    let default_genesis = cfg.get_genesis()?;
    let genesis = match custom {
        Some(c) => {
            let patch = from_str(&c)?;
            let mut base = from_str(&default_genesis)?;
            merge(&mut base, &patch);
            info!("customized genesis config");
            to_string_pretty(&base)?
        },
        None => default_genesis,
    };
    write_file(&path, &genesis)?;
    Ok(())
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::ConfigApp { chain, output, custom }) => {
            config_app(&chain, output, custom).unwrap();
        },
        Some(Commands::ConfigTendermint { chain, output, custom, moniker }) => {
            config_tendermint(&chain, output, custom, moniker).unwrap();
        },
        Some(Commands::ConfigGenesis { chain, output, custom }) => {
            config_genesis(&chain, output, custom).unwrap();
        },
        None => {
            error!("missing command");
        },
    }
}
