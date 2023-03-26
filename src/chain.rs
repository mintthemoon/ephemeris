use std::collections::HashMap;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Clone, Deserialize, Serialize)]
pub struct Chain {
    pub name: String,
    pub chain_id: String,
    pub minimum_gas_prices: String,
    pub genesis_url: String,
    pub rpcs: Vec<String>,
    pub seeds: Vec<String>,
    pub docker_image: String,
}

impl Chain {
    pub fn default() -> Self {
        Chain {
            name: "".to_string(),
            chain_id: "".to_string(),
            minimum_gas_prices: "0stake".to_string(),
            genesis_url: "".to_string(),
            rpcs: vec![],
            seeds: vec![],
            docker_image: "".to_string(),
        }
    }

    pub fn from_id(id: &str) -> Result<Self> {
        CHAINS.get(id).map(Self::clone).ok_or(anyhow!("no config for chain '{}'", id))
    }
}

const CHAINS_JSON: &str = include_str!("../data/chains.json");

lazy_static! {
    pub static ref CHAINS: HashMap<String, Chain> = {
        let chain_list: Vec<Chain> = from_str(CHAINS_JSON).unwrap();
        chain_list.into_iter().map(|c| (c.chain_id.clone(), c)).collect()
    };
}