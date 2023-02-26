mod config;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
    },
    /// configure config.toml
    ConfigTendermint {
        /// chain id
        #[arg(short, long)]
        chain: String,
        /// output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// configure genesis.json
    ConfigGenesis {
        /// chain id
        #[arg(short, long)]
        chain: String,
        /// output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::ConfigApp { chain, output }) => {
            let cfg = default_config(chain).unwrap();
            let path = match output {
                Some(o) => o.join("app.toml"),
                None => PathBuf::new().join("app.toml"),
            };
            cfg.render_app_config(&path).unwrap();
        },
        Some(Commands::ConfigTendermint { chain, output }) => {
            let cfg = default_config(chain).unwrap();
            let path = match output {
                Some(o) => o.join("config.toml"),
                None => PathBuf::new().join("config.toml"),
            };
            cfg.render_tendermint_config(&path).unwrap();
        },
        Some(Commands::ConfigGenesis { chain, output }) => {
            let cfg = default_config(chain).unwrap();
            let path = match output {
                Some(o) => o.join("genesis.json"),
                None => PathBuf::new().join("genesis.json"),
            };
            cfg.download_genesis(&path).unwrap();
        },
        None => {},
    }
}
