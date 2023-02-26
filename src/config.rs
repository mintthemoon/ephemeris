use std::io::Read;

use anyhow::{Result, anyhow};
use askama::Template;
use reqwest::blocking::get;
use serde::{Serialize, Deserialize};
use flate2::read::GzDecoder;

#[derive(Serialize, Deserialize)]
pub struct CosmosTelemetryConfig {
    pub service_name: String,
    pub enabled: bool,
    pub enable_hostname: bool,
    pub enable_hostname_label: bool,
    pub enable_service_label: bool,
    pub prometheus_retention_time: u64,
    pub global_labels: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize)]
pub struct CosmosApiConfig {
    pub enable: bool,
    pub swagger: bool,
    pub address: String,
    pub max_open_connections: u64,
    pub rpc_read_timeout: u64,
    pub rpc_write_timeout: u64,
    pub rpc_max_body_bytes: u64,
    pub enabled_unsafe_cors: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CosmosRosettaConfig {
    pub enable: bool,
    pub address: String,
    pub blockchain: String,
    pub network: String,
    pub retries: u64,
    pub offline: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CosmosGrpcConfig {
    pub enable: bool,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct CosmosGrpcWebConfig {
    pub enable: bool,
    pub address: String,
    pub enable_unsafe_cors: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CosmosStateSyncConfig {
    pub snapshot_interval: u64,
    pub snapshot_keep_recent: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CosmosWasmConfig {
    pub query_gas_limit: u64,
    pub lru_size: u64,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "cosmos_app.toml", escape = "none")]
pub struct CosmosAppConfig {
    pub minimum_gas_prices: String,
    pub pruning: String,
    pub pruning_keep_recent: u64,
    pub pruning_keep_every: u64,
    pub pruning_interval: u64,
    pub halt_height: u64,
    pub halt_time: u64,
    pub min_retain_blocks: u64,
    pub inter_block_cache: bool,
    pub index_events: Vec<String>,
    pub iavl_cache_size: u64,
    pub iavl_disable_fastnode: bool,
    pub telemetry: CosmosTelemetryConfig,
    pub api: CosmosApiConfig,
    pub rosetta: CosmosRosettaConfig,
    pub grpc: CosmosGrpcConfig,
    pub grpc_web: CosmosGrpcWebConfig,
    pub state_sync: CosmosStateSyncConfig,
    pub wasm: Option<CosmosWasmConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintRpcConfig {
    pub laddr: String,
    pub cors_allowed_origins: Vec<String>,
    pub cors_allowed_methods: Vec<String>,
    pub cors_allowed_headers: Vec<String>,
    pub grpc_laddr: String,
    pub grpc_max_open_connections: u64,
    #[serde(rename = "unsafe")]
    pub allow_unsafe: bool,
    pub max_open_connections: u64,
    pub max_subscription_clients: u64,
    pub max_subscriptions_per_client: u64,
    pub subscription_buffer_size: u64,
    pub websocket_write_buffer_size: u64,
    pub close_on_slow_client: bool,
    pub timeout_broadcast_tx_commit: String,
    pub max_body_bytes: u64,
    pub max_header_bytes: u64,
    pub tls_cert_file: String,
    pub tls_key_file: String,
    pub pprof_laddr: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintP2pConfig {
    pub laddr: String,
    pub external_address: String,
    pub seeds: String,
    pub persistent_peers: String,
    pub upnp: bool,
    pub addr_book_file: String,
    pub addr_book_strict: bool,
    pub max_num_inbound_peers: u64,
    pub max_num_outbound_peers: u64,
    pub unconditional_peer_ids: String,
    pub persistent_peers_max_dial_period: String,
    pub flush_throttle_timeout: String,
    pub max_packet_msg_payload_size: u64,
    pub send_rate: u64,
    pub recv_rate: u64,
    pub pex: bool,
    pub seed_mode: bool,
    pub private_peer_ids: String,
    pub allow_duplicate_ip: bool,
    pub handshake_timeout: String,
    pub dial_timeout: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintMempoolConfig {
    pub version: String,
    pub recheck: bool,
    pub broadcast: bool,
    pub wal_dir: String,
    pub size: u64,
    pub max_txs_bytes: u64,
    pub cache_size: u64,
    pub keep_invalid_txs_in_cache: bool,
    pub max_tx_bytes: u64,
    pub max_batch_bytes: u64,
    pub ttl_duration: String,
    pub ttl_num_blocks: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintStatesyncConfig {
    pub enable: bool,
    pub rpc_servers: Vec<String>,
    pub trust_height: u64,
    pub trust_hash: String,
    pub trust_period: String,
    pub discovery_time: String,
    pub temp_dir: String,
    pub chunk_request_timeout: String,
    pub chunk_fetchers: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintFastsyncConfig {
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintConsensusConfig {
    pub wal_file: String,
    pub timeout_propose: String,
    pub timeout_propose_delta: String,
    pub timeout_prevote: String,
    pub timeout_prevote_delta: String,
    pub timeout_precommit: String,
    pub timeout_precommit_delta: String,
    pub timeout_commit: String,
    pub double_sign_check_height: u64,
    pub skip_timeout_commit: bool,
    pub create_empty_blocks: bool,
    pub create_empty_blocks_interval: String,
    pub peer_gossip_sleep_duration: String,
    pub peer_query_maj23_sleep_duration: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintStorageConfig {
    pub discard_abci_responses: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintTransactionIndexConfig {
    pub indexer: String,
    pub psql_conn: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintInstrumentationConfig {
    pub prometheus: bool,
    pub prometheus_listen_addr: String,
    pub max_open_connections: u64,
    pub namespace: String,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tendermint_config.toml", escape = "none")]
pub struct TendermintConfig {
    pub proxy_app: String,
    pub moniker: String,
    pub fast_sync: bool,
    pub db_backend: String,
    pub db_dir: String,
    pub log_level: String,
    pub log_format: String,
    pub genesis_file: String,
    pub priv_validator_key_file: String,
    pub priv_validator_state_file: String,
    pub priv_validator_laddr: String,
    pub node_key_file: String,
    pub abci: String,
    pub filter_peers: bool,
    pub rpc: TendermintRpcConfig,
    pub p2p: TendermintP2pConfig,
    pub mempool: TendermintMempoolConfig,
    pub statesync: TendermintStatesyncConfig,
    pub consensus: TendermintConsensusConfig,
    pub fastsync: TendermintFastsyncConfig,
    pub storage: TendermintStorageConfig,
    pub tx_index: TendermintTransactionIndexConfig,
    pub instrumentation: TendermintInstrumentationConfig,
}

#[derive(Serialize, Deserialize)]
pub struct CosmosChainConfig {
    pub app: CosmosAppConfig,
    pub tendermint: TendermintConfig,
    pub genesis_url: String,
}

impl CosmosChainConfig {
    pub fn get_app_config(&self) -> Result<String> {
        self.app.render().map_err(anyhow::Error::from)
    }

    pub fn get_tendermint_config(&self) -> Result<String> {
        self.tendermint.render().map_err(anyhow::Error::from)
    }

    pub fn get_genesis(&self) -> Result<String> {
        if self.genesis_url == "" {
            return Err(anyhow!("no genesis URL configured"));
        }
        if self.genesis_url.ends_with(".gz") {
            let mut d = GzDecoder::new(get(&self.genesis_url)?);
            let mut s = String::new();
            d.read_to_string(&mut s)?;
            Ok(s)
        } else {
            get(&self.genesis_url)?.text().map_err(anyhow::Error::from)
        }
    }
}

pub fn default_wasmd_config() -> CosmosChainConfig {
    CosmosChainConfig {
        app: CosmosAppConfig {
            minimum_gas_prices: "0stake".to_string(),
            pruning: "nothing".to_string(),
            pruning_keep_recent: 0,
            pruning_keep_every: 0,
            pruning_interval: 0,
            halt_height: 0,
            halt_time: 0,
            min_retain_blocks: 0,
            inter_block_cache: true,
            index_events: vec![],
            iavl_cache_size: 781250,
            iavl_disable_fastnode: false,
            telemetry: CosmosTelemetryConfig {
                service_name: "".to_string(),
                enabled: false,
                enable_hostname: false,
                enable_hostname_label: false,
                enable_service_label: false,
                prometheus_retention_time: 0,
                global_labels: vec![],
            },
            api: CosmosApiConfig {
                enable: true,
                swagger: false,
                address: "tcp://127.0.0.1:1317".to_string(),
                max_open_connections: 1000,
                rpc_read_timeout: 10,
                rpc_write_timeout: 10,
                rpc_max_body_bytes: 1000000,
                enabled_unsafe_cors: true,
            },
            rosetta: CosmosRosettaConfig {
                enable: false,
                address: "".to_string(),
                blockchain: "".to_string(),
                network: "".to_string(),
                retries: 0,
                offline: false,
            },
            grpc: CosmosGrpcConfig {
                enable: true,
                address: "tcp://127.0.0.1:9090".to_string(),
            },
            grpc_web: CosmosGrpcWebConfig {
                enable: true,
                address: "tcp://127.0.0.1:9091".to_string(),
                enable_unsafe_cors: true
            },
            state_sync: CosmosStateSyncConfig {
                snapshot_interval: 2000,
                snapshot_keep_recent: 3
            },
            wasm: Some(CosmosWasmConfig {
                query_gas_limit: 30000000,
                lru_size: 0
            }),
        },
        tendermint: TendermintConfig {
            proxy_app: "tcp://127.0.0.1:26658".to_string(),
            moniker: "".to_string(),
            fast_sync: true,
            db_backend: "goleveldb".to_string(),
            db_dir: "data".to_string(),
            log_level: "info".to_string(),
            log_format: "plain".to_string(),
            genesis_file: "config/genesis.json".to_string(),
            priv_validator_key_file: "config/priv_validator_key.json".to_string(),
            priv_validator_state_file: "data/priv_validator_state.json".to_string(),
            priv_validator_laddr: "".to_string(),
            node_key_file: "config/node_key.json".to_string(),
            abci: "socket".to_string(),
            filter_peers: false,
            rpc: TendermintRpcConfig {
                laddr: "tcp://127.0.0.1:26657".to_string(),
                cors_allowed_origins: vec!["*".to_string()],
                cors_allowed_methods: vec![],
                cors_allowed_headers: vec![],
                grpc_laddr: "".to_string(),
                grpc_max_open_connections: 900,
                allow_unsafe: false,
                max_open_connections: 900,
                max_subscription_clients: 100,
                max_subscriptions_per_client: 5,
                subscription_buffer_size: 200,
                websocket_write_buffer_size: 200,
                close_on_slow_client: false,
                timeout_broadcast_tx_commit: "10s".to_string(),
                max_body_bytes: 1000000,
                max_header_bytes: 1048576,
                tls_cert_file: "".to_string(),
                tls_key_file: "".to_string(),
                pprof_laddr: "localhost:6060".to_string(),
            },
            p2p: TendermintP2pConfig {
                laddr: "tcp://0.0.0.0:26656".to_string(),
                external_address: "".to_string(),
                seeds: "".to_string(),
                persistent_peers: "".to_string(),
                upnp: false,
                addr_book_file: "".to_string(),
                addr_book_strict: true,
                max_num_inbound_peers: 50,
                max_num_outbound_peers: 50,
                unconditional_peer_ids: "".to_string(),
                persistent_peers_max_dial_period: "0s".to_string(),
                flush_throttle_timeout: "100ms".to_string(),
                max_packet_msg_payload_size: 1024,
                send_rate: 5120000,
                recv_rate: 5120000,
                pex: true,
                seed_mode: false,
                private_peer_ids: "".to_string(),
                allow_duplicate_ip: false,
                handshake_timeout: "20s".to_string(),
                dial_timeout: "3s".to_string(),
            },
            mempool: TendermintMempoolConfig {
                version: "v0".to_string(),
                recheck: true,
                broadcast: true,
                wal_dir: "".to_string(),
                size: 5000,
                max_txs_bytes: 1073741824,
                cache_size: 10000,
                keep_invalid_txs_in_cache: false,
                max_tx_bytes: 1048576,
                max_batch_bytes: 0,
                ttl_duration: "".to_string(),
                ttl_num_blocks: 0,
            },
            statesync: TendermintStatesyncConfig {
                enable: false,
                rpc_servers: vec![],
                trust_height: 0,
                trust_hash: "".to_string(),
                trust_period: "".to_string(),
                discovery_time: "15s".to_string(),
                temp_dir: "".to_string(),
                chunk_request_timeout: "10s".to_string(),
                chunk_fetchers: 4
            },
            fastsync: TendermintFastsyncConfig { version: "v0".to_string() },
            consensus: TendermintConsensusConfig {
                wal_file: "data/cs.wal/wal".to_string(),
                timeout_propose: "3s".to_string(),
                timeout_propose_delta: "500ms".to_string(),
                timeout_prevote: "1s".to_string(),
                timeout_prevote_delta: "500ms".to_string(),
                timeout_precommit: "1s".to_string(),
                timeout_precommit_delta: "500ms".to_string(),
                timeout_commit: "5s".to_string(),
                double_sign_check_height: 0,
                skip_timeout_commit: false,
                create_empty_blocks: true,
                create_empty_blocks_interval: "0s".to_string(),
                peer_gossip_sleep_duration: "100ms".to_string(),
                peer_query_maj23_sleep_duration: "2s".to_string(),
            },
            storage: TendermintStorageConfig { discard_abci_responses: false },
            tx_index: TendermintTransactionIndexConfig { indexer: "kv".to_string(), psql_conn: "".to_string() },
            instrumentation: TendermintInstrumentationConfig {
                prometheus: false,
                prometheus_listen_addr: ":26660".to_string(),
                max_open_connections: 3,
                namespace: "tendermint".to_string(),
            },
        },
        genesis_url: "".to_string(),
    }
}

pub fn default_config(chain_id: &str) -> Option<CosmosChainConfig> {
    match chain_id {
        "kaiyo-1" => {
            let mut cfg = default_wasmd_config();
            cfg.app.minimum_gas_prices = "0.00119ukuji,0.00150factory/kujira1qk00h5atutpsv900x202pxx42npjr9thg58dnqpa72f2p7m2luase444a7/uusk,0.00150ibc/295548A78785A1007F232DE286149A6FF512F180AF5657780FC89C009E2C348F,0.000125ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2,0.00126ibc/47BD209179859CDE4A2806763D7189B6E6FE13A17880FE2B42DE1E6C1E329E23,0.00652ibc/3607EB5B5E64DD1C0E12E07F077FF470D5BC4706AFCBC98FE1BA960E5AE4CE07,617283951ibc/F3AA7EF362EC5E791FE78A0F4CCC69FEE1F9A7485EB1A8CAB3F6601C00522F10,0.000288ibc/EFF323CC632EC4F747C61BCE238A758EFDB7699C3226565F7C20DA06509D59A5,0.000125ibc/DA59C009A0B3B95E0549E6BF7B075C8239285989FF457A8EDDBB56F10B2A6986,0.00137ibc/A358D7F19237777AF6D8AD0E0F53268F8B18AE8A53ED318095C14D6D7F3B2DB5,0.0488ibc/4F393C3FCA4190C0A6756CE7F6D897D5D1BE57D6CCB80D0BC87393566A7B6602,78492936ibc/004EBF085BBED1029326D56BE8A2E67C08CECE670A94AC1947DF413EF5130EB2,964351ibc/1B38805B1C75352B28169284F96DF56BDEBD9E8FAC005BDCC8CF0378C82AA8E7".to_string();
            cfg.tendermint.consensus.timeout_commit = "1500ms".to_string();
            cfg.genesis_url = "https://raw.githubusercontent.com/Team-Kujira/networks/master/mainnet/kaiyo-1.json".to_string();
            Some(cfg)
        },
        "harpoon-4" => {
            let mut cfg = default_wasmd_config();
            cfg.app.minimum_gas_prices = "0.00125ukuji".to_string();
            cfg.tendermint.consensus.timeout_commit = "1500ms".to_string();
            cfg.genesis_url = "https://raw.githubusercontent.com/Team-Kujira/networks/master/testnet/harpoon-4.json".to_string();
            Some(cfg)
        },
        "cosmoshub-4" => {
            let mut cfg = default_wasmd_config();
            cfg.app.minimum_gas_prices = "0.0025uatom".to_string();
            cfg.app.wasm = None;
            cfg.genesis_url = "https://raw.githubusercontent.com/cosmos/mainnet/master/genesis/genesis.cosmoshub-4.json.gz".to_string();
            Some(cfg)
        },
        "theta-testnet-001" => {
            let mut cfg = default_wasmd_config();
            cfg.app.minimum_gas_prices = "0.0025uatom".to_string();
            cfg.app.wasm = None;
            cfg.genesis_url = "https://github.com/cosmos/testnets/raw/master/public/genesis.json.gz".to_string();
            Some(cfg)
        },
        _ => None,
    }
}