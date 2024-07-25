# Stylus-EVM-Precompiles

The repository contains playground for testing EVM Precompiles in Stylus smart contracts:

-   [ecrecover](https://docs.soliditylang.org/en/latest/units-and-global-variables.html#mathematical-and-cryptographic-functions)

## Build

```sh
cargo stylus check
```

## Export ABI

```sh
cargo stylus export-abi
```

## Deploy

```sh
cargo stylus deploy \
  --private-key-path=<path-to-priv-key>
```
