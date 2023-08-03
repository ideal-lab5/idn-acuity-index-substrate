# Hybrid Indexer Project Announcement

Hybrid is a Rust software library for indexing events in Substrate blockchains. It stores the bare minimum in its database to identify events containing certain parameters, e.g. `AccountId`, and should be used in tandem with a light client or full node to actually retrieve event contents. This is why it is called "Hybrid".

Hybrid can either be used in a decentralized or centralized manner:

* decentralized - A dapp can run the indexer directly on the user's device. It can be configured to only index certain block ranges and event parameters to minimize resource usage such as time, bandwidth and storage space.

* centralized - The indexer can be run in a data center. Dapps can use it for chain queries that are not possible via light client or WSS connection to a full node.

Currently hybrid connects directly to a full node to index it. In a later version it will be able to index a chain via a light client. 

Development of this tool was funded by a [grant](https://github.com/w3f/Grants-Program/blob/master/applications/hybrid.md) from the Web3 Foundation.

## Architecture

![Hybrid Architecture](https://raw.githubusercontent.com/ethernomad/hybrid-diagram/main/hybrid.png)

A Hybrid indexer binary has to be built for each chain type in a similar manner to how a full node binary using the Substrate library has to be built. For example, polkadot-indexer indexes events on chains supported by the polkadot binary (Polkadot, Kusama, Rococo and Westend).

It reads events in blocks using subxt and indexes these events in a Key-value database using the sled library. This is considerably more efficient than storing the index in an SQL database.

Events that have identifying parameters will be indexed. For example the Transfer event in the Balances pallet is identifiable by the `AccountId` of both `from` and `to`.

Additionally, all events are indexed by event variant.

To index a block, first a query has to be made to determine the hash from the block number. Then a second query for the metadata version. Finally the block itself is downloaded. In order to ensure throughput is as high as possible, multiple blocks are indexed simultaneously to counteract the round-trip delay.

When a chain is going to potentially perform a runtime upgrade, the Hybrid Indexer for the chain will need a new release with any updated events. If an instance of the indexer is not updated before the runtime upgrade occurs, it can be restarted with the new version at the correct block number.

WSS queries are handled via the highly scalable [tokio_tungstenite](https://github.com/snapview/tokio-tungstenite) Rust library.

It is possible to subscribe to queries so that the dapp will be altered as soon as the event is emitted.

The database keys are constructed in such a way that events can be found using iterators starting at a specific block number. For example, for for the AccountId keyspace:

`AccountId/BlockNumber/EventIndex`

Database entries are key-only. No value is stored. The blocknumber and event index are all that need to be returned for each event found. This reduces the size of the index database and increases decentralization. The frontend can query the chain in a decentralized manner to retrieve the event.
