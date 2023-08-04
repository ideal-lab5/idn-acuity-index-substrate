# Milestone 3 Testing Guide

## Deliverable 1 - Hybrid Indexer Library

Check out this repository and run the tests:

 ```sh
git clone https://github.com/hybrid-explorer/hybrid-indexer
cd hybrid-indexer
cargo test
```

## Deliverable 2 - Polkadot Indexer

1. Follow the [instructions](https://github.com/hybrid-explorer/blob/main/README.md#docker) in polkadot-indexer to run a docker image in a separate console tab for each of polkadot, kusama, rococo & westend.
1. Observe that all 4 Polakdot chains are being indexed.

## Deliverable 3 - Chain Select

1. Follow the [instructions](https://github.com/hybrid-explorer/hybrid-dapp/blob/main/README.md#docker) in hybrid-dapp to run a docker image and launch the dapp in a web browser.
1. If the chain selector is not visible in the top left, click on the "hamburger" icon.
1. Switch between the different chains.
1. Observe that the indexing status at the top changes for each chain.

For each chain check that the indexing is working:
1. Select Pallet / Variant search key.
1. Select Balances pallet.
1. Select Transfer variant.
1. Click Search.
1. Observe events are found.
1. Copy an account address from an event to the clipboard.
1. Select AccountId search key.
1. Paste account address into AccountId box.
1. Click Search.
1. Observe events are found.
