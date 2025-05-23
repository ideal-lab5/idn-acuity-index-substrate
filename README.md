# Acuity Index Substrate
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

It reads events in all blocks using [subxt](https://github.com/paritytech/subxt) and indexes these events in a key-value database using the [sled](http://sled.rs/) library. This is considerably more efficient than storing the index in an SQL database.

Events that have identifying parameters will be indexed. For example the Transfer event in the Balances pallet is identifiable by the `AccountId` of both `from` and `to`.

Hybrid has built-in indexing macros for the following Substrate pallets: System, Preimage, Indices, Balances, Transaction Payment, Staking, Session, Democracy, Collective, Elections Phragmen, Treasury, Vesting, Identity, Proxy, Multisig, Fast Unstake, Election Provider Multi-phase, Tips, Bounties, Child Bounties, Bags List, Nomination Pools.

Hybrid currently supports indexing of the following event parameters: `AccountId`, `AccountIndex`, `AuctionIndex`, `BountyIndex`, `CandidateHash`, `EraIndex`, `MessageId`, `ParaId`, `PoolId`, `PreimageHash`, `ProposalHash`, `RefIndex`, `RegistrarIndex`, `SessionIndex`, `TipHash`.

## IDeal Network Extensions

In addition to the standard Substrate pallets, this fork adds support for Ideal Network (IDN) specific pallets and event parameters. The following IDN-specific features have been implemented:

- **IDN Manager Pallet**: Support for indexing subscription-related events and tracking subscription IDs.
- **Randomness Beacon Pallet**: Support for indexing beacon events, pulse rounds, and beacon public keys.

Additional event parameters that are now indexed:
- `SubscriptionId`: Track subscription-related events by their unique identifier
- `PulseRound`: Index events by their randomness pulse round number
- `BeaconPublicKey`: Index events related to specific beacon public keys

The indexer is configured to work with the IDN runtime and handle both standard Substrate events and IDN-specific events. This allows for efficient querying of IDN-related operations through the same WSS interface.

Additionally, all events are indexed by event variant. This means that, for example, a list of all balance transfers for all accounts can be obtained. 

To index a block, first a query has to be made to determine the hash from the block number. Then a second query for the metadata version. Finally the block itself is downloaded. In order to ensure throughput is as high as possible, multiple blocks are indexed simultaneously to counteract the round-trip delay.

In the same manner that each Substrate chain is a separate Rust build that uses Substrate crates, each chain will need a separate Hybrid Indexer build that is configured to index the correct pallets.

When a chain is going to potentially perform a runtime upgrade, the Hybrid Indexer for the chain will need a new release with any updated events. If an instance of the indexer is not updated before the runtime upgrade occurs, it can be restarted with the new version at the correct block number.

WSS queries are handled via the highly scalable [tokio_tungstenite](https://github.com/snapview/tokio-tungstenite) Rust library.

Consumers will be able to subscribe for new events that match a query.

The database keys are constructed in such a way so that events can be found using iterators starting at a specific block number. For example, for for the AccountId keyspace:

`AccountId/BlockNumber/EventIndex`

Database entries are key-only. No value is stored. The blocknumber and event index are all that need to be returned for each event found. This reduces the size of the index database and increases decentralization. The frontend can query the chain in a decentralized manner to retrieve the event.
