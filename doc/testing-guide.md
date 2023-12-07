# Grant 2 Milestone 3 Testing Guide

To run the unit tests:

```
git clone https://github.com/hybrid-explorer/hybrid-indexer
cd hybrid-indexer
rustup default nightly
cargo test
```

## Deliverable 1 - Support additional indexes

Observe that the RuntimeIndexer trait now has a [ChainKey](https://github.com/hybrid-explorer/hybrid-indexer/blob/main/src/shared.rs#L42) Associated type:

ChainKey must implement the [IndexKey](https://github.com/hybrid-explorer/hybrid-indexer/blob/main/src/shared.rs#L338) trait. This type is used to read and write custom keys. It has the ChainTrees associated type that implements the [IndexTrees](https://github.com/hybrid-explorer/hybrid-indexer/blob/main/src/shared.rs#L67) trait.

IndexTrees is used to open and flush the database trees for custom keys.

The [tutorial](https://github.com/hybrid-explorer/hybrid-indexer/blob/main/doc/tutorial.md) and [API](https://github.com/hybrid-explorer/hybrid-indexer/blob/main/doc/api.md) docs have been updated to reflect these changes.

[Tests](https://github.com/hybrid-explorer/hybrid-indexer/blob/main/src/tests.rs#L563) have been written for custom keys.

## Deliverable 2 - Variant index optional

Build and enter the docker image:

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

Observe that event variant indexing is disabled:

```
📇 Event variant indexing: disabled
```

Keys/sec is typically less than events/sec.

Press ctrl+c to stop the indexer.

Run polkadot-indexer again with variant indexing enabled:

```
./target/release/polkadot-indexer --index-variant
```

```
📇 Event variant indexing: enabled
```

Keys/sec is equal to or more than events/sec because every event has at least the variant key.

Observe that the previous span of indexed blocks is re-indexed.

Press ctrl+c to stop the indexer.

## Deliverable 3 - Expose cache_capacity() and mode()

Run polkadot-indexer:

```
./target/release/polkadot-indexer
```

Observe the database mode and cache capacity:

```
Database mode: LowSpace
Database cache capacity: 1024.00 MiB
```

Run polkadot-indexer with different database settings:

```
./target/release/polkadot-indexer --db-cache-capacity 0.5GiB --db-mode high-throughput
```

Observe the database mode and cache capacity:

```
Database mode: HighThroughput
Database cache capacity: 512.00 MiB
```
