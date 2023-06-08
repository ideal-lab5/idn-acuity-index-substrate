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

Observe that at block #13800016 new metadata will be downloaded because a runtime upgrade has occured.

Using the dapp, test the following search queries and verify the results:

```
AccountId: 5CszgdfkARHQAgr8rMVQV2v9trkgZ77ksuign4sinvYa66B2
15104642, balances, Withdraw, who: 5CszgdfkARHQAgr8rMVQV2v9trkgZ77ksuign4sinvYa66B2, amount: 121000000
15104642, balances, Transfer, from: 5EYCAe5ijiYfyeZ2JJEYsk8UzApweYacAt5zgjz1FMQbykPU, to: 5CszgdfkARHQAgr8rMVQV2v9trkgZ77ksuign4sinvYa66B2, value: 189304306673
15104642, childBounties, Claimed, index: 11, childIndex: 187, payout: 189304306673, beneficiary: 5CszgdfkARHQAgr8rMVQV2v9trkgZ77ksuign4sinvYa66B2
15104642, transactionPayment, TransactionFeePaid, who: 5CszgdfkARHQAgr8rMVQV2v9trkgZ77ksuign4sinvYa66B2, actualFee: 121000000, tip: 0
```

```
AccountIndex: 9494
10013701, indices, IndexFreed, index: 9494
```

```
AuctionIndex: 15
10018925, auctions, WinningOffset, auctionIndex: 15, blockNumber: 3377
10018925, auctions, AuctionClosed, auctionIndex: 15
```

```
BountyIndex: 11
15104642, childBounties, Claimed, index: 11, childIndex: 187, payout: 189304306673, beneficiary: 5CszgdfkARHQAgr8rMVQV2v9trkgZ77ksuign4sinvYa66B2
```

```
CandidateHash: 0x6a1cd467afb69aa2b23866538b1160a60d96228587c5d7efc1d3c1ce4e3efb63
10059744, parasDisputes, DisputeInitiated, candidate_hash: 0x6a1cd467afb69aa2b23866538b1160a60d96228587c5d7efc1d3c1ce4e3efb63, dispute_location: local
10059744, parasDisputes, DisputeConcluded, candidate_hash: 0x6a1cd467afb69aa2b23866538b1160a60d96228587c5d7efc1d3c1ce4e3efb63, dispute_location: valid
```

```
MessageId: 0xc656c0814b4174d3fbae7b0dd3ae63a94ac858b9120f8dc13027d2ee89f54a46 
15100192, ump, ExecutedUpward, id: 0xc656c0814b4174d3fbae7b0dd3ae63a94ac858b9120f8dc13027d2ee89f54a46
```

```
ParaId: 2013
10018925, slots, Leased, paraId: 2013, leaser: 5EYCAe5ijiYdg22N9CfytScFVQsZ9tKaH8GwQKcCtrisZvAb, periodBegin: 8, periodCount: 8, extraReserved: 9336595339185988, totalAmount: 9336595339185988
```

```
PoolId: 12
15180584, nominationPools, PaidOut, member: 5HpXUP5QYvsDuTA2Y7SCRtJ66fE21Kv76CxHx3gM4w2Y51CG, poolId: 12, payout: 21784733850
15180584, nominationPools, Bonded, member: 5HpXUP5QYvsDuTA2Y7SCRtJ66fE21Kv76CxHx3gM4w2Y51CG, poolId: 12, bonded: 21784733850, joined: false
```

```
ProposalHash: 0x7c403355a3747fea8a84968a7a83b7f5d2b26ea0b5d63b317ae65c1b091cf07b
10025666, collective, Voted, account: 5FH76VkU2cfKSpxxHxdc53cTnVADJtdmgZ7hPZFHd15KVt4m, proposalHash: 0x7c403355a3747fea8a84968a7a83b7f5d2b26ea0b5d63b317ae65c1b091cf07b, voted: true, yes: 3, no: 0
```

```
ProposalIndex: 103
10022400, treasury, Awarded, proposalIndex: 103, award: 39509700000000, account: 5HpTYRjg7XHrhUW4NTDAL9xJi7iw7e1UBUYghnH9eHUK8GeH
```

```
RefIndex: 114
15100839, democracy, Voted, voter: 5G78rS6hYFeAr9Cb49NcXU2zEsL3Z7qQg1JCrYwea6kV8KFS, refIndex: 114
```

```
RegistrarIndex: 1
10027254, identity, JudgementRequested, who: 5C7WyVoJGo9NLjHuK9bvgyoytWsDqSgzMdAX4n6nWSruk2dE, registrarIndex: 1
```

```
TipHash: 0x729c6a740112abfc8cd143771f1f88518c3906e86f601a6c6a312fe7f7babf33
10146463, tips, NewTip, tip_hash: 0x729c6a740112abfc8cd143771f1f88518c3906e86f601a6c6a312fe7f7babf33
```

Currently, not all Polkadot events are indexed. Of the events that are indexed, some event details will not be displayed. AccountIds are displayed as generic Substrate addresses, not Polkadot addresses. This is because in milestone 2, the events will not be stored in the index and will be loaded from the blockchain in the frontend.

