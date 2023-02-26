mod config;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use serde_json::{from_value, from_str, to_value, to_string, to_string_pretty, Value};
use json_patch::merge;
use log::{info, error};
use gethostname::gethostname;

use crate::config::{default_config, default_wasmd_config};

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
        chain: Option<String>,
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
        chain: Option<String>,
        /// output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// parameter overrides json
        #[arg(long)]
        custom: Option<String>,
        /// node moniker
        #[arg(short, long)]
        moniker: Option<String>,
    },
    /// configure genesis.json
    ConfigGenesis {
        /// chain id
        #[arg(short, long)]
        chain: Option<String>,
        /// output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// parameter overrides json
        #[arg(long)]
        custom: Option<String>,
        /// custom genesis url
        #[arg(short, long)]
        genesis_url: Option<String>,
    },
    // configure all supported chain files
    Config {
        /// chain id
        #[arg(short, long)]
        chain: Option<String>,
        /// output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// parameter overrides json
        #[arg(long)]
        custom: Option<String>,
        /// node moniker
        #[arg(short, long)]
        moniker: Option<String>,
        /// custom genesis url
        #[arg(short, long)]
        genesis_url: Option<String>,
    }
}

fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    File::create(path)?.write_all(&content.as_bytes())?;
    info!("wrote {}", path.to_string_lossy());
    Ok(())
}

fn config_app(chain: &Option<String>, output: &Option<PathBuf>, custom: &Option<String>) -> Result<()> {
    let default_cfg = match chain {
        Some(c) => default_config(c).ok_or(anyhow!("chain not supported: {}", c))?,
        None => default_wasmd_config(),
    };
    let cfg = match custom {
        Some(c) => {
            let patch = from_str(&format!("{{\"app\": {}}}", c))?;
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

fn config_tendermint(chain: &Option<String>, output: &Option<PathBuf>, custom: &Option<String>, moniker: &Option<String>) -> Result<()> {
    let default_cfg = match chain {
        Some(c) => default_config(c).ok_or(anyhow!("chain not supported: {}", c))?,
        None => default_wasmd_config(),
    };
    let mut cfg = match custom {
        Some(c) => {
            let patch = from_str(&format!("{{\"tendermint\": {}}}", c))?;
            let mut base = to_value(default_cfg)?;
            merge(&mut base, &patch);
            info!("customized tendermint config");
            from_value(base)?
        },
        None => default_cfg,
    };
    match moniker {
        Some(m) => { cfg.tendermint.moniker = m.clone(); },
        None => { cfg.tendermint.moniker = gethostname().into_string().unwrap_or("node".to_string()); }
    }
    let path = match output {
        Some(o) => o.join("config.toml"),
        None => PathBuf::new().join("config.toml"),
    };
    write_file(&path, &cfg.get_tendermint_config()?)?;
    Ok(())
}

fn config_genesis(chain: &Option<String>, output: &Option<PathBuf>, custom: &Option<String>, genesis_url: &Option<String>) -> Result<()> {
    let mut cfg = match chain {
        Some(c) => default_config(c).ok_or(anyhow!("chain not supported: {}", c))?,
        None => default_wasmd_config(),
    };
    let path = match output {
        Some(o) => o.join("genesis.json"),
        None => PathBuf::new().join("genesis.json"),
    };
    genesis_url.as_ref().map(|url| cfg.genesis_url = url.clone());
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

fn config(chain: &Option<String>, output: &Option<PathBuf>, custom: &Option<String>, moniker: &Option<String>, genesis_url: &Option<String>) -> Result<()> {
    let customs = match custom {
        Some(c) => {
            let patch: Value = from_str(c)?;
            (
                patch.get("app").map(to_string).transpose()?,
                patch.get("tendermint").map(to_string).transpose()?,
                patch.get("genesis").map(to_string).transpose()?,
            )
        },
        None => (None, None, None)
    };
    config_app(chain, output, &customs.0)?;
    config_tendermint(chain, output, &customs.1, moniker)?;
    config_genesis(chain, output, &customs.2, genesis_url)?;
    Ok(())
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::ConfigApp { chain, output, custom }) => {
            config_app(chain, output, custom).unwrap();
        },
        Some(Commands::ConfigTendermint { chain, output, custom, moniker }) => {
            config_tendermint(chain, output, custom, moniker).unwrap();
        },
        Some(Commands::ConfigGenesis { chain, output, custom, genesis_url }) => {
            config_genesis(chain, output, custom, genesis_url).unwrap();
        },
        Some(Commands::Config { chain, output, custom, moniker, genesis_url }) => {
            config(chain, output, custom, moniker, genesis_url).unwrap();
        },
        None => {
            error!("missing command");
        },
    }
}
