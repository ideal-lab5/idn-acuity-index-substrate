# Grant 2 Milestone 1 Testing Guide

To run the unit tests:

```
git clone https://github.com/hybrid-explorer/hybrid-indexer
cd hybrid-indexer
rustup default nightly
cargo test
```

## Deliverable 1 - Combine head and batch indexer threads

Observe that both head and batch indexing is now handled by the same event loop:
https://github.com/hybrid-explorer/hybrid-indexer/blob/main/src/substrate.rs#L636

Run polkadot-indexer starting at block 1752000:

```
git clone https://github.com/hybrid-explorer/polkadot-indexer/
cd polkadot-indexer
docker build .
docker run --rm -p 8172:8172 [image_hash] -c polkadot -b 17520000 -p 8172
```

(Replace `[image_hash]` with the hash at the end of the build step.)

Observe that "Downloading metadata for spec version 9430" only appears once.

## Deliverable 2 - Check correct chain

Attempt to index Polkadot, but connect to a Kusama end point:

```
docker run --rm -p 8172:8172 [image_hash] -c polkadot -p 8172 -u wss://kusama-rpc.polkadot.io:443
```

Observe that the indexer detects that the genesis hash is wrong.

## Deliverable 3 - Improved logging

Run the indexer:

```
docker run --rm -p 8172:8172 [image_hash] -c polkadot -b 17520000 -p 8172
```

Observe that the batch indexing stats are output every 2 seconds.

Run the indexer in quiet mode:

```
docker run --rm -p 8172:8172 [image_hash] -c polkadot -b 17520000 -p 8172 -q
```

Observe there is no output.

Run the indexer in verbose mode:

```
docker run --rm -p 8172:8172 [image_hash] -c polkadot -b 17520000 -p 8172 -v
```

Observe there is a lot of output.

## Deliverable 4 - Improved error checking

Observe the new `IndexError` enum: 

https://github.com/hybrid-explorer/hybrid-indexer/blob/main/src/shared.rs#L11

Run the indexer:

```
docker run --rm -p 8172:8172 [image_hash] -c polkadot -b 17520000 -p 8172
```

Press ctrl+c. Observe that the indexer exits gracefully, closing the database.

Run the indexer on an invalid block:

```
docker run --rm -p 8172:8172 [image_hash] -c polkadot -b 20000000 -p 8172
```

Observe that the error is reported and batch indexing stops.
