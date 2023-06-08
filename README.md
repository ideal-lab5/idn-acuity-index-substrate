# hybrid-indexer
Substrate event indexer.

Development of this tool was funded by a [grant](https://github.com/w3f/Grants-Program/blob/master/applications/hybrid.md) from the Web3 Foundation.

## Overview

Hybrid takes a unique, partially decentralized approach that improves two major problems with current open source Substrate block explorers: centralization and huge hosting requirements.

A fully centralized block explorer typically populates an SQL database with the entirety of an archive node and stores additional data to index everything. Operating such a database reliably requires huge system resources and expense.

When querying block information, or the chain state at any block height, the Hybrid dapp will use the [Substrate Connect](https://substrate.io/developers/substrate-connect/) light client from within the browser. Alternatively, these queries can be made directly to an archive node via WSS.

For event search functionality, the Hybrid indexer efficiently indexes events in all blocks so they can be found with a simple WSS query. For example, to find all events connected with a specific `AccountId`.

This architecture has three main advantages:
- state queries are fully decentralized - you don't have to trust an RPC provider not to lie to you
- 100% availability - the light client doesn't depend on any centralized service that may not always be available
- the Hybrid indexer has significantly lower system requirements - it doesn't need to store all chain history

Eventually, Hybrid will use this centralized / decentralized approach as the basis for an ink! contract explorer.

Because Substrate is a federated platform, it will be possible browse multiple chains from the Hybrid dapp.

### Architecture

![Hybrid Architecture](https://raw.githubusercontent.com/ethernomad/hybrid-diagram/main/hybrid.png)

The Hybrid indexer is written in Rust. It can be configured to connect to any Substrate chain.

It reads events in all blocks using [subxt](https://github.com/paritytech/subxt) and index these events in a Key-value database using the [sled](http://sled.rs/) library. This is considerably more efficient than storing the index in an SQL database.

subxt currently has an [issue](https://github.com/paritytech/subxt/issues/793#issuecomment-1386902010) where it is not possible to query blocks prior to V14 metadata (block #7,229,126 on Polkadot). Resolving this issue is not within the scope of the grant. Once this grant is completed a further grant application will be made that includes resolving this issue.

When decoding events, subxt needs to have the correct metadata. The metadata changes whenever a chain performs a runtime upgrade. Hybrid Indexer handles this in a very elegant way. When indexing begins it downloads the metadata for the starting block. When it encounters a decoding error it downloads the metadata for the current block and retries decoding. This means that the indexer does not have to be built with the metadata and block number of every runtime upgrade.

To index an event, it needs to be converted into a Rust type that matches the metadata. Sometimes the metadata for an event will change during a runtime upgrade. To handle this the indexer will have Rust types for current and historic versions of all events. When an event fails to be converted into a Rust type the previous type will be tried.

All events in all pallets that have identifying parameters will be indexed. For example the Transfer event in the Balances pallet is identifiable by the `AccountId` of both `from` and `to`.

Other examples of identifying event parameters are `assetId` in the Assets pallet, `code_hash` in the contracts pallet, `CollectionId` and `ItemId` in the NFTs pallet, and `MultiLocation` in the XCM pallet.

Additionally, all events are indexed by event variant.

To download a block, a query first has to be made to determine the hash for a given block number. In order to ensure throughput is as high as possible, multiple queries to the full node will be active at the same time to avoid round-trip delay. Block processing will be in a separate thread. 

In the same manner that each Substrate chain is a separate Rust build that uses Substrate crates, each chain will need a separate Hybrid Indexer build that is configured to index the correct pallets.

When a chain is going to potentially perform a runtime upgrade, the Hybrid Indexer for the chain will need a new release with any updated events. If an instance of the indexer is not updated before the runtime upgrade occurs, it can be restarted with the new version at the correct block number.

WSS queries will be handled via the highly scalable [tokio_tungstenite](https://github.com/snapview/tokio-tungstenite) Rust library.

In addition to the identifier being searched for, queries will be able to include start block, offset, and limit to control which events are returned.

Consumers will be able to subscribe for new events that match a query.

The database keys will be constructed in such a way so that events can be found using iterators starting at a specific block number. For example, for for the AccountId keyspace:

`AccountId/BlockNumber/EventIndex`

Database entries will be key-only. No value will be stored. The blocknumber and event index are all that need to be returned for each event found. This reduces the size of the index database and increases decentralization. The frontend can query the chain in a decentralized manner to retrieve the event.

### Tutorial

Hybrid is still early in the development process. Currently it can index event parameters from many events in Polkadot: AccountId, AccountIndex, AuctionIndex, BountyIndex, CandidateHash, EraIndex, MessageId, ParaId, PoolId, PreimageHash, ProposalHash, ProposalIndex, RefIndex, RegistrarIndex, SessionIndex, TipHash. Additionally, all events are indexed by event variant.

#### Run tests

```
cargo test
```

#### Build

```
cargo build --release
```

#### Run indexer

```
./target/release/hybrid-indexer --block-height 15870000 
```

The earlist block that can be indexed on Polkadot is 7,229,126.

The indexer simultaneously indexes historical blocks (batch) and finalized blocks (head).

When the indexer starts it will start batch indexing from where indexing finished last time it was run. This can be overidden with the --block-height parameter.

Head blocks are always indexed as they are finalized. Once batch indexing has caught up with head it will stop and only blocks being finalized will be indexed.

#### Run the dapp

Go to [hybrid-dapp](https://github.com/hybrid-explorer/hybrid-dapp/tree/milestone-1) and follow the tutorial to run the frontend to query the indexer.

### Docker

After setting up Docker, run the following command to build the image:

```
docker build .
```

Then run the image with the correct port mapping:

```
docker run --rm -p 8172:8172 [image_hash]
```

Now run the dockerfile for [hybrid-dapp](https://github.com/hybrid-explorer/hybrid-dapp/tree/milestone-1).

### Testing Guide

Ensure that you have both the the indexer and dapp running, either by following the above tutorial, or by following the Docker instructions.

#### Deliverable 2.1 & 2.2

Event subscription API / Live dapp

1. Select Pallet / Variant search key.
2. Select Balances pallet.
3. Select Transfer variant.
4. Click Search.
5. Observe that as new balance transfers occur on Polkadot, they automatically appear in the results.

#### Deliverable 2.3

preimage_hash, era_index and session_index are now indexed in addition to the keys in milestone 1.

Using the dapp, test the following search queries and verify the results:

```
PreimageHash: 0xdb2b6cb38c2f6704ed067da2e9001bc57314be4f0117f664a93c0d18610110c5
15764612	preimage	Noted	A preimage has been noted.
hash_: 0xdb2b6cb38c2f6704ed067da2e9001bc57314be4f0117f664a93c0d18610110c5
```

```
EraIndex: 1076
15825858	fastUnstake	BatchChecked	A batch was partially checked for the given eras, but the process did not finish.
eras: [ "1,076" ]
```

```
SessionIndex: 6552
15649648	session	NewSession	New session has happened. Note that the argument is the session index, not the block number as the type might suggest.
sessionIndex: 6,552
```

#### Deliverable 2.4

All events are now indexed by their variant.

1. Select Pallet / Variant search key.
2. Select System pallet.
3. Select NewAccount variant.
4. Click Search.
5. Observe that events with the selected variant are found.

#### Deliverable 2.5

Event contents are now loaded directly from the chain, rather than from the indexer.

Observe that when searching for events, the details column is populated with information about the event.
