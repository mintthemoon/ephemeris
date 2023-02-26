use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use askama::Template;
use reqwest::blocking::get;
use serde::{Serialize, Deserialize};

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
pub struct WasmdWasmConfig {
    pub query_gas_limit: u64,
    pub lru_size: u64,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "wasmd_app.toml", escape = "none")]
pub struct WasmdAppConfig {
    minimum_gas_prices: String,
    pruning: String,
    pruning_keep_recent: u64,
    pruning_keep_every: u64,
    pruning_interval: u64,
    halt_height: u64,
    halt_time: u64,
    min_retain_blocks: u64,
    inter_block_cache: bool,
    index_events: Vec<String>,
    iavl_cache_size: u64,
    iavl_disable_fastnode: bool,
    telemetry: CosmosTelemetryConfig,
    api: CosmosApiConfig,
    rosetta: CosmosRosettaConfig,
    grpc: CosmosGrpcConfig,
    grpc_web: CosmosGrpcWebConfig,
    state_sync: CosmosStateSyncConfig,
    wasm: WasmdWasmConfig,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintRpcConfig {
    laddr: String,
    cors_allowed_origins: Vec<String>,
    cors_allowed_methods: Vec<String>,
    cors_allowed_headers: Vec<String>,
    grpc_laddr: String,
    grpc_max_open_connections: u64,
    #[serde(rename = "unsafe")]
    allow_unsafe: bool,
    max_open_connections: u64,
    max_subscription_clients: u64,
    max_subscriptions_per_client: u64,
    subscription_buffer_size: u64,
    websocket_write_buffer_size: u64,
    close_on_slow_client: bool,
    timeout_broadcast_tx_commit: String,
    max_body_bytes: u64,
    max_header_bytes: u64,
    tls_cert_file: String,
    tls_key_file: String,
    pprof_laddr: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintP2pConfig {
    laddr: String,
    external_address: String,
    seeds: String,
    persistent_peers: String,
    upnp: bool,
    addr_book_file: String,
    addr_book_strict: bool,
    max_num_inbound_peers: u64,
    max_num_outbound_peers: u64,
    unconditional_peer_ids: String,
    persistent_peers_max_dial_period: String,
    flush_throttle_timeout: String,
    max_packet_msg_payload_size: u64,
    send_rate: u64,
    recv_rate: u64,
    pex: bool,
    seed_mode: bool,
    private_peer_ids: String,
    allow_duplicate_ip: bool,
    handshake_timeout: String,
    dial_timeout: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintMempoolConfig {
    version: String,
    recheck: bool,
    broadcast: bool,
    wal_dir: String,
    size: u64,
    max_txs_bytes: u64,
    cache_size: u64,
    keep_invalid_txs_in_cache: bool,
    max_tx_bytes: u64,
    max_batch_bytes: u64,
    ttl_duration: String,
    ttl_num_blocks: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintStatesyncConfig {
    enable: bool,
    rpc_servers: Vec<String>,
    trust_height: u64,
    trust_hash: String,
    trust_period: String,
    discovery_time: String,
    temp_dir: String,
    chunk_request_timeout: String,
    chunk_fetchers: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintFastsyncConfig {
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintConsensusConfig {
    wal_file: String,
    timeout_propose: String,
    timeout_propose_delta: String,
    timeout_prevote: String,
    timeout_prevote_delta: String,
    timeout_precommit: String,
    timeout_precommit_delta: String,
    timeout_commit: String,
    double_sign_check_height: u64,
    skip_timeout_commit: bool,
    create_empty_blocks: bool,
    create_empty_blocks_interval: String,
    peer_gossip_sleep_duration: String,
    peer_query_maj23_sleep_duration: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintStorageConfig {
    discard_abci_responses: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintTransactionIndexConfig {
    indexer: String,
    psql_conn: String,
}

#[derive(Serialize, Deserialize)]
pub struct TendermintInstrumentationConfig {
    prometheus: bool,
    prometheus_listen_addr: String,
    max_open_connections: u64,
    namespace: String,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tendermint_config.toml", escape = "none")]
pub struct TendermintConfig {
    proxy_app: String,
    moniker: String,
    fast_sync: bool,
    db_backend: String,
    db_dir: String,
    log_level: String,
    log_format: String,
    genesis_file: String,
    priv_validator_key_file: String,
    priv_validator_state_file: String,
    priv_validator_laddr: String,
    node_key_file: String,
    abci: String,
    filter_peers: bool,
    rpc: TendermintRpcConfig,
    p2p: TendermintP2pConfig,
    mempool: TendermintMempoolConfig,
    statesync: TendermintStatesyncConfig,
    consensus: TendermintConsensusConfig,
    fastsync: TendermintFastsyncConfig,
    storage: TendermintStorageConfig,
    tx_index: TendermintTransactionIndexConfig,
    instrumentation: TendermintInstrumentationConfig,
}

#[derive(Serialize, Deserialize)]
pub struct CosmwasmChainConfig {
    pub app_config: WasmdAppConfig,
    pub tendermint_config: TendermintConfig,
    pub genesis_url: String,
}

impl CosmwasmChainConfig {
    pub fn render_app_config(&self, path: &PathBuf) -> Result<()> {
        File::create(path)?.write_all(self.app_config.render()?.as_bytes())?;
        Ok(())
    }

    pub fn render_tendermint_config(&self, path: &PathBuf) -> Result<()> {
        File::create(path)?.write_all(self.tendermint_config.render()?.as_bytes())?;
        Ok(())
    }

    pub fn download_genesis(&self) -> Result<String> {
        get(&self.genesis_url)?.text().map_err(anyhow::Error::from)
    }
}

pub fn default_wasmd_config() -> CosmwasmChainConfig {
    CosmwasmChainConfig {
        app_config: WasmdAppConfig {
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
            wasm: WasmdWasmConfig {
                query_gas_limit: 30000000,
                lru_size: 0
            },
        },
        tendermint_config: TendermintConfig {
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

pub fn default_config(chain_id: &str) -> Option<CosmwasmChainConfig> {
    match chain_id {
        "kaiyo-1" => {
            let mut cfg = default_wasmd_config();
            cfg.app_config.minimum_gas_prices = "0.00119ukuji,0.00150factory/kujira1qk00h5atutpsv900x202pxx42npjr9thg58dnqpa72f2p7m2luase444a7/uusk,0.00150ibc/295548A78785A1007F232DE286149A6FF512F180AF5657780FC89C009E2C348F,0.000125ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2,0.00126ibc/47BD209179859CDE4A2806763D7189B6E6FE13A17880FE2B42DE1E6C1E329E23,0.00652ibc/3607EB5B5E64DD1C0E12E07F077FF470D5BC4706AFCBC98FE1BA960E5AE4CE07,617283951ibc/F3AA7EF362EC5E791FE78A0F4CCC69FEE1F9A7485EB1A8CAB3F6601C00522F10,0.000288ibc/EFF323CC632EC4F747C61BCE238A758EFDB7699C3226565F7C20DA06509D59A5,0.000125ibc/DA59C009A0B3B95E0549E6BF7B075C8239285989FF457A8EDDBB56F10B2A6986,0.00137ibc/A358D7F19237777AF6D8AD0E0F53268F8B18AE8A53ED318095C14D6D7F3B2DB5,0.0488ibc/4F393C3FCA4190C0A6756CE7F6D897D5D1BE57D6CCB80D0BC87393566A7B6602,78492936ibc/004EBF085BBED1029326D56BE8A2E67C08CECE670A94AC1947DF413EF5130EB2,964351ibc/1B38805B1C75352B28169284F96DF56BDEBD9E8FAC005BDCC8CF0378C82AA8E7".to_string();
            cfg.tendermint_config.consensus.timeout_commit = "1500ms".to_string();
            cfg.genesis_url = "https://raw.githubusercontent.com/Team-Kujira/networks/master/mainnet/kaiyo-1.json".to_string();
            Some(cfg)
        },
        "harpoon-4" => {
            let mut cfg = default_wasmd_config();
            cfg.app_config.minimum_gas_prices = "0.00125ukuji".to_string();
            cfg.tendermint_config.consensus.timeout_commit = "1500ms".to_string();
            cfg.genesis_url = "https://raw.githubusercontent.com/Team-Kujira/networks/master/testnet/harpoon-4.json".to_string();
            Some(cfg)
        },
        _ => None,
    }
}