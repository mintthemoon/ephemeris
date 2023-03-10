mod config;
mod rpc;

use std::path::PathBuf;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::env;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use serde_json::{from_value, from_str, to_value, to_string, to_string_pretty, Value};
use json_patch::merge;
use log::{info, error};
use gethostname::gethostname;

use crate::config::{default_config, default_wasmd_config};
use crate::rpc::BlockingRpc;

#[derive(Parser)]
#[command(name = "starsign", author = "mintthemoon <mint@mintthemoon.xyz>", version = "0.1.4")]
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
        /// enable statesync
        #[arg(short, long)]
        statesync: bool,
        /// custom statesync rpc
        #[arg(long)]
        statesync_rpc: Option<String>,
        /// custom statesync snapshot interval (default 2000)
        #[arg(long)]
        statesync_interval: Option<u64>,
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
        /// use existing genesis file
        #[arg(long)]
        genesis_file: Option<PathBuf>,
    },
    /// configure all supported chain files
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
        /// use existing genesis file
        #[arg(long)]
        genesis_file: Option<PathBuf>,
        /// enable statesync
        #[arg(short, long)]
        statesync: bool,
        /// custom statesync rpc
        #[arg(long)]
        statesync_rpc: Option<String>,
        /// custom statesync snapshot interval (default 2000)
        #[arg(long)]
        statesync_interval: Option<u64>,
    },
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

fn config_tendermint(
    chain: &Option<String>, output: &Option<PathBuf>, custom: &Option<String>, moniker: &Option<String>, statesync: &bool, statesync_rpc: &Option<String>, statesync_interval: &Option<u64>,
) -> Result<()> {
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
    info!("using moniker {}", cfg.tendermint.moniker);
    if *statesync {
        let rpc_url = match statesync_rpc {
            Some(r) => {
                cfg.tendermint.statesync.rpc_servers = vec![r.clone(), r.clone()];
                r
            },
            None => {
                if cfg.tendermint.statesync.rpc_servers.is_empty() {
                    return Err(anyhow!("statesync enabled but no rpc servers are configured"));
                }
                if cfg.tendermint.statesync.rpc_servers.len() == 1 {
                    cfg.tendermint.statesync.rpc_servers.push(cfg.tendermint.statesync.rpc_servers[0].clone())
                }
                &cfg.tendermint.statesync.rpc_servers[0]
            }
        };
        let rpc = BlockingRpc::from_url(rpc_url)?;
        let sync_info = rpc.status()?.sync_info;
        let latest_height = sync_info.latest_block_height.value();
        let snapshot_interval = statesync_interval.unwrap_or(2000);
        let snapshot_height = (latest_height / snapshot_interval) * snapshot_interval;
        let trust_hash = rpc.block(snapshot_height.try_into()?)?.block_id.hash.to_string();
        cfg.tendermint.statesync.enable = true;
        cfg.tendermint.statesync.trust_height = snapshot_height;
        cfg.tendermint.statesync.trust_hash = trust_hash.clone();
        info!("enabled statesync to height {} ({}) from {}", snapshot_height, trust_hash, rpc_url);
    }
    let path = match output {
        Some(o) => o.join("config.toml"),
        None => PathBuf::new().join("config.toml"),
    };
    write_file(&path, &cfg.get_tendermint_config()?)?;
    Ok(())
}

fn config_genesis(
    chain: &Option<String>, output: &Option<PathBuf>, custom: &Option<String>, genesis_url: &Option<String>, genesis_file: &Option<PathBuf>,
) -> Result<()> {
    let mut cfg = match chain {
        Some(c) => default_config(c).ok_or(anyhow!("chain not supported: {}", c))?,
        None => default_wasmd_config(),
    };
    genesis_url.as_ref().map(|url| cfg.genesis_url = url.clone());
    let default_genesis = match genesis_file {
        Some(f) => read_to_string(f)?,
        None => cfg.get_genesis()?,
    };
    let path = match output {
        Some(o) => o.join("genesis.json"),
        None => PathBuf::new().join("genesis.json"),
    };
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

fn config(
    chain: &Option<String>, output: &Option<PathBuf>, custom: &Option<String>, moniker: &Option<String>, statesync: &bool, statesync_rpc: &Option<String>, statesync_interval: &Option<u64>, genesis_url: &Option<String>, genesis_file: &Option<PathBuf>,
) -> Result<()> {
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
    config_tendermint(chain, output, &customs.1, moniker, statesync, statesync_rpc, statesync_interval)?;
    config_genesis(chain, output, &customs.2, genesis_url, genesis_file)?;
    Ok(())
}

fn cli_start() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::ConfigApp { chain, output, custom }) => {
            config_app(chain, output, custom)
        },
        Some(Commands::ConfigTendermint { 
            chain, output, custom, moniker, statesync, statesync_rpc, statesync_interval,
        }) => {
            config_tendermint(chain, output, custom, moniker, statesync, statesync_rpc, statesync_interval)
        },
        Some(Commands::ConfigGenesis { chain, output, custom, genesis_url, genesis_file }) => {
            config_genesis(chain, output, custom, genesis_url, genesis_file)
        },
        Some(Commands::Config {
            chain, output, custom, moniker, statesync, statesync_rpc, statesync_interval, genesis_url, genesis_file,
        }) => {
            config(chain, output, custom, moniker, statesync, statesync_rpc, statesync_interval, genesis_url, genesis_file)
        },
        None => {
            Err(anyhow!("missing command"))
        },
    }
}

fn main() -> Result<()> {
    env::var("RUST_LOG").err().map(|_| env::set_var("RUST_LOG", "info"));
    env_logger::init();
    cli_start().map_err(|err| { error!("configuration failed: {}", err); err })
}
