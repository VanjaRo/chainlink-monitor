# Chainlink price monitor

## About

Logging script, tracking price changes on chainlink token pairs:

```
1. USDT/ETH - 0x7De0d6fce0C128395C488cb4Df667cdbfb35d7DE
2. USDC/ETH - 0xe5BbBdb2Bb953371841318E1Edfbf727447CeF2E
3. LINK/ETH - 0xbba12740DE905707251525477bAD74985DeC46D2
```

The implementation uses `AnswerUpdated` events on the aggregator contracts as a signal.  
Script connects to Etherium node using WebScoket, utilizing provided **PROVIDER_WS_URL** env variable in _.env_ file.

## Build

**MAKE SURE** you've created _.env_ file with url to Etherium mainnet WS connection (you can use Infura/Alchemy for that)

To build the script –– run:

```
cargo build -r
```

## Run

To run binaries –– run:

```
./target/release/chainlink-price-monitor
```

## Output Example

```
token pair: USDC/ETH, new price: 830775190000000, timestamp: 1668072728
token pair: USDT/ETH, new price: 839947760000000, timestamp: 1668072839
...
```
