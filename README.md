# starsign
Cosmos node configuration, simplified. 🚀🪐🌍

## Introduction
Starsign manages many of the routine steps in setting up a node with simple, reusable, and endlessly customizable commands. Supported chains have default settings applied automatically and any others can be configured with the powerful `--custom` flag.

Supported files include `app.toml`, `config.toml`, and `genesis.json`.

**WARNING**: This software is not fully tested and may contain some bugs. Back up your configs and use at your own risk!

## Installation
### Compile with cargo
```bash
cargo install --git https://github.com/mintthemoon/starsign
```

### Run with docker
```bash
docker run -v $HOME/.kujira:/config ghcr.io/mintthemoon/starsign config -o /config <...>
```

## Configure your node
### Supported chain
```bash
starsign config -c kaiyo-1 -o $HOME/.kujira/config --statesync
```
Run `starsign config -h` for a full list of options.

### Custom chain
```bash
starsign config -o $HOME/.osmosisd/config \
    --genesis-url https://github.com/osmosis-labs/networks/raw/main/osmosis-1/genesis.json \
    --custom '{"app": {"minimum-gas-prices": "0.0025uosmo"}}' \
    --moniker <moniker>
```
Run `starsign config -h` for a full list of options.


### Individual files
```bash
starsign config-app -c kaiyo-1 --custom '{"pruning": "everything"}'
starsign config-tendermint -c kaiyo-1 -m <moniker>
```
Run `starsign -h` for a full list of supported actions.

### Existing genesis
Rather than downloading `genesis.json` from a URL, provide a path to an existing file which you can customize. Useful for initializing local chains.
```bash
starsign config-genesis --genesis-file /path/to/genesis.json \
    --custom '{"app_state": {"wasm": {"params": {"instantiate_default_permission": "Everybody"}}}}'
```

## Supported chains
| Chain | Type | ID |
| ----- | ---- | -- |
| Kujira | Mainnet | `kaiyo-1` |
| Kujira | Testnet | `harpoon-4` |
| Gaia | Mainnet | `cosmoshub-4` |
| Gaia | Testnet | `theta-testnet-001` |
