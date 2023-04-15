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

Hybrid is still early in the development process. Currently it can index event parameters from many events in Polkadot: AccountId, AccountIndex, AuctionIndex, BountyIndex, CandidateHash, MessageId, ParaId, PoolId, ProposalHash, ProposalIndex, RefIndex, RegistrarIndex, TipHash.

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
./target/release/hybrid-indexer --url wss://rpc.polkadot.io:443 --block-height 13800000 
```

All parameters that are indexed will be printed on the console, except AccountId.

The earlist block that can be indexed on Polkadot is 7,229,126.

If the --block-height parameter is not specified it will resume from where it left off in the previous run.

#### Run the dapp

Go to [hybrid-dapp](https://github.com/hybrid-explorer/hybrid-dapp) and follow the tutorial to run the frontend to query the indexer.

### Docker

After setting up Docker, run the following command to build the image:

```
docker build .
```

Then run the image with the correct port mapping:

```
docker run --rm -p 8172:8172 [image_hash]
```

Now run the dockerfile for [hybrid-dapp](https://github.com/hybrid-explorer/hybrid-dapp).
