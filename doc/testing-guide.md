# Grant 2 Milestone 2 Testing Guide

To run the unit tests:

```
git clone https://github.com/hybrid-explorer/hybrid-indexer
cd hybrid-indexer
rustup default nightly
cargo test
```

## Deliverable 1 - Index backwards

Run the docker image:

```
git clone https://github.com/hybrid-explorer/polkadot-indexer/
cd polkadot-indexer
docker build .
docker run -it [image_hash] /bin/bash
```

(Replace `[image_hash]` with the hash at the end of the build step.)

Run polkadot-indexer:

```
./target/release/polkadot-indexer  
```

Observe that batch indexing starts at the head block and works backwards.

Press ctrl+c to stop the indexer.

## Deliverable 2 - Store indexed spans

Run polkadot-indexer again:

```
./target/release/polkadot-indexer  
```

Observe that it detects the previous span of indexed blocks and skips over it.

Press ctrl+c to stop the indexer.

## Deliverable 3 - Declare indexer start blocks

Run polkadot-indexer again with a higher indexer version:

```
./target/release/polkadot-indexer -c polkadot2
```

Observe that it detects the previous span of indexed blocks and re-indexes it.

Press ctrl+c to stop the indexer.
