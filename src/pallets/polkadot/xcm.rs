use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum XcmPallet {
    #[serde(rename_all = "camelCase")]
    Attempted(xcm::latest::Outcome),
    /// A XCM message was sent.
    ///
    /// \[ origin, destination, message \]
    #[serde(rename_all = "camelCase")]
    Sent(MultiLocation, MultiLocation, Xcm<()>),
    /// Query response received which does not match a registered query. This may be because a
    /// matching query was never registered, it may be because it is a duplicate response, or
    /// because the query timed out.
    ///
    /// \[ origin location, id \]
    #[serde(rename_all = "camelCase")]
    UnexpectedResponse(MultiLocation, QueryId),
    /// Query response has been received and is ready for taking with `take_response`. There is
    /// no registered notification call.
    ///
    /// \[ id, response \]
    #[serde(rename_all = "camelCase")]
    ResponseReady(QueryId, Response),
    /// Query response has been received and query is removed. The registered notification has
    /// been dispatched and executed successfully.
    ///
    /// \[ id, pallet index, call index \]
    #[serde(rename_all = "camelCase")]
    Notified(QueryId, u8, u8),
    /// Query response has been received and query is removed. The registered notification could
    /// not be dispatched because the dispatch weight is greater than the maximum weight
    /// originally budgeted by this runtime for the query result.
    ///
    /// \[ id, pallet index, call index, actual weight, max budgeted weight \]
    #[serde(rename_all = "camelCase")]
    NotifyOverweight(QueryId, u8, u8, Weight, Weight),
    /// Query response has been received and query is removed. There was a general error with
    /// dispatching the notification call.
    ///
    /// \[ id, pallet index, call index \]
    #[serde(rename_all = "camelCase")]
    NotifyDispatchError(QueryId, u8, u8),
    /// Query response has been received and query is removed. The dispatch was unable to be
    /// decoded into a `Call`; this might be due to dispatch function having a signature which
    /// is not `(origin, QueryId, Response)`.
    ///
    /// \[ id, pallet index, call index \]
    #[serde(rename_all = "camelCase")]
    NotifyDecodeFailed(QueryId, u8, u8),
    /// Expected query response has been received but the origin location of the response does
    /// not match that expected. The query remains registered for a later, valid, response to
    /// be received and acted upon.
    ///
    /// \[ origin location, id, expected location \]
    #[serde(rename_all = "camelCase")]
    InvalidResponder(MultiLocation, QueryId, Option<MultiLocation>),
    /// Expected query response has been received but the expected origin location placed in
    /// storage by this runtime previously cannot be decoded. The query remains registered.
    ///
    /// This is unexpected (since a location placed in storage in a previously executing
    /// runtime should be readable prior to query timeout) and dangerous since the possibly
    /// valid response will be dropped. Manual governance intervention is probably going to be
    /// needed.
    ///
    /// \[ origin location, id \]
    #[serde(rename_all = "camelCase")]
    InvalidResponderVersion(MultiLocation, QueryId),
    /// Received query response has been read and removed.
    ///
    /// \[ id \]
    #[serde(rename_all = "camelCase")]
    ResponseTaken(QueryId),
    /// Some assets have been placed in an asset trap.
    ///
    /// \[ hash, origin, assets \]
    #[serde(rename_all = "camelCase")]
    AssetsTrapped(H256, MultiLocation, VersionedMultiAssets),
    /// An XCM version change notification message has been attempted to be sent.
    ///
    /// The cost of sending it (borne by the chain) is included.
    ///
    /// \[ destination, result, cost \]
    #[serde(rename_all = "camelCase")]
    VersionChangeNotified(MultiLocation, XcmVersion, MultiAssets),
    /// The supported version of a location has been changed. This might be through an
    /// automatic notification or a manual intervention.
    ///
    /// \[ location, XCM version \]
    #[serde(rename_all = "camelCase")]
    SupportedVersionChanged(MultiLocation, XcmVersion),
    /// A given location which had a version change subscription was dropped owing to an error
    /// sending the notification to it.
    ///
    /// \[ location, query ID, error \]
    #[serde(rename_all = "camelCase")]
    NotifyTargetSendFail(MultiLocation, QueryId, XcmError),
    /// A given location which had a version change subscription was dropped owing to an error
    /// migrating the location to our new XCM format.
    ///
    /// \[ location, query ID \]
    #[serde(rename_all = "camelCase")]
    NotifyTargetMigrationFail(VersionedMultiLocation, QueryId),
    /// Expected query response has been received but the expected querier location placed in
    /// storage by this runtime previously cannot be decoded. The query remains registered.
    ///
    /// This is unexpected (since a location placed in storage in a previously executing
    /// runtime should be readable prior to query timeout) and dangerous since the possibly
    /// valid response will be dropped. Manual governance intervention is probably going to be
    /// needed.
    ///
    /// \[ origin location, id \]
    #[serde(rename_all = "camelCase")]
    InvalidQuerierVersion(MultiLocation, QueryId),
    /// Expected query response has been received but the querier location of the response does
    /// not match the expected. The query remains registered for a later, valid, response to
    /// be received and acted upon.
    ///
    /// \[ origin location, id, expected querier, maybe actual querier \]
    #[serde(rename_all = "camelCase")]
    InvalidQuerier(MultiLocation, QueryId, MultiLocation, Option<MultiLocation>),
    /// A remote has requested XCM version change notification from us and we have honored it.
    /// A version information message is sent to them and its cost is included.
    ///
    /// \[ destination location, cost \]
    #[serde(rename_all = "camelCase")]
    VersionNotifyStarted(MultiLocation, MultiAssets),
    /// We have requested that a remote chain sends us XCM version change notifications.
    ///
    /// \[ destination location, cost \]
    #[serde(rename_all = "camelCase")]
    VersionNotifyRequested(MultiLocation, MultiAssets),
    /// We have requested that a remote chain stops sending us XCM version change notifications.
    ///
    /// \[ destination location, cost \]
    #[serde(rename_all = "camelCase")]
    VersionNotifyUnrequested(MultiLocation, MultiAssets),
    /// Fees were paid from a location for an operation (often for using `SendXcm`).
    ///
    /// \[ paying location, fees \]
    #[serde(rename_all = "camelCase")]
    FeesPaid(MultiLocation, MultiAssets),
    /// Some assets have been claimed from an asset trap
    ///
    /// \[ hash, origin, assets \]
    #[serde(rename_all = "camelCase")]
    AssetsClaimed(H256, MultiLocation, VersionedMultiAssets),
}

pub fn pallet_xcm_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Claimed" => {
            let event = event.as_event::<polkadot::pallet_xcm::events::Claimed>()?.unwrap();
            let event_db = Event::XcmPallet(
                XcmPallet::Claimed {
                    who: event.who.clone(),
                    ethereum_address: event.ethereum_address.0,
                    amount: event.amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees, event.who, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
