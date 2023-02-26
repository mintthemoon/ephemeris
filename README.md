# ephemeris
Cosmos node configuration, simplified. 🚀🪐🌍

## Introduction
Ephemeris manages many of the routine steps in setting up a node with simple, reusable, and endlessly customizable commands. Supported chains have default settings applied automatically and any others can be configured with the powerful `--custom` flag.

Supported files include `app.toml`, `config.toml`, and `genesis.json`.

## Configure your node
### Supported chain
```bash
ephemeris config -c kaiyo-1 -o $HOME/.kujira/config
```
Run `ephemeris config -h` for a full list of options.

### Custom chain
```bash
ephemeris config -o $HOME/.osmosisd/config \
    --genesis-url https://github.com/osmosis-labs/networks/raw/main/osmosis-1/genesis.json \
    --custom '{"app": {"minimum-gas-prices": "0.0025uosmo"}}' \
    --moniker <moniker>
```
Run `ephemeris config -h` for a full list of options.


### Individual files
```bash
ephemeris config-app -c kaiyo-1 --custom '{"pruning": "everything"}'
ephemeris config-tendermint -c kaiyo-1 -m <moniker>
```
Run `ephemeris -h` for a full list of supported actions.

### Existing genesis
Rather than downloading `genesis.json` from a URL, provide a path to an existing file which you can customize. Useful for initializing local chains.
```bash
ephemeris config-genesis --genesis-file /path/to/genesis.json \
    --custom '{"app_state": {"wasm": {"params": {"instantiate_default_permission": "Everybody"}}}}'
```

## Supported chains
| Chain | Type | ID |
| ----- | ---- | -- |
| Kujira | Mainnet | `kaiyo-1` |
| Kujira | Testnet | `harpoon-4` |
| Gaia | Mainnet | `cosmoshub-4` |
| Gaia | Testnet | `theta-testnet-001` |
