#[macro_export]
macro_rules! index_system_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::NewAccount { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            <$event_enum>::KilledAccount { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_preimage_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Noted { hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            <$event_enum>::Requested { hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            <$event_enum>::Cleared { hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_indices_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::IndexAssigned { who, index } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
                $indexer.index_event_account_index(index, $block_number, $event_index);
            }
            <$event_enum>::IndexFreed { index } => {
                $indexer.index_event_account_index(index, $block_number, $event_index);
            }
            <$event_enum>::IndexFrozen { index, who } => {
                $indexer.index_event_account_index(index, $block_number, $event_index);
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_balances_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Endowed { account, .. } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            <$event_enum>::DustLost { account, .. } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            <$event_enum>::Transfer { from, to, .. } => {
                $indexer.index_event_account_id(from, $block_number, $event_index);
                $indexer.index_event_account_id(to, $block_number, $event_index);
            }
            <$event_enum>::BalanceSet { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Reserved { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Unreserved { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::ReserveRepatriated { from, to, .. } => {
                $indexer.index_event_account_id(from, $block_number, $event_index);
                $indexer.index_event_account_id(to, $block_number, $event_index);
            }
            <$event_enum>::Deposit { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Withdraw { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Slashed { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Minted { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Burned { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Suspended { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Restored { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Upgraded { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Locked { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Unlocked { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Frozen { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Thawed { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_transaction_payment_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::TransactionFeePaid { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_staking_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::EraPaid { era_index, .. } => {
                $indexer.index_event_era_index(era_index, $block_number, $event_index);
            }
            <$event_enum>::Rewarded { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index);
            }
            <$event_enum>::Slashed { staker, .. } => {
                $indexer.index_event_account_id(staker, $block_number, $event_index);
            }
            <$event_enum>::SlashReported {
                validator,
                fraction: _,
                slash_era,
            } => {
                $indexer.index_event_account_id(validator, $block_number, $event_index);
                $indexer.index_event_era_index(slash_era, $block_number, $event_index);
            }
            <$event_enum>::OldSlashingReportDiscarded { session_index } => {
                $indexer.index_event_session_index(session_index, $block_number, $event_index);
            }
            <$event_enum>::Bonded { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index);
            }
            <$event_enum>::Unbonded { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index);
            }
            <$event_enum>::Withdrawn { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index);
            }
            <$event_enum>::Kicked { nominator, stash } => {
                $indexer.index_event_account_id(nominator, $block_number, $event_index);
                $indexer.index_event_account_id(stash, $block_number, $event_index);
            }
            <$event_enum>::Chilled { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index);
            }
            <$event_enum>::PayoutStarted {
                era_index,
                validator_stash,
            } => {
                $indexer.index_event_era_index(era_index, $block_number, $event_index);
                $indexer.index_event_account_id(validator_stash, $block_number, $event_index);
            }
            <$event_enum>::ValidatorPrefsSet { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_session_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::NewSession { session_index } => {
                $indexer.index_event_session_index(session_index, $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_democracy_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Proposed { proposal_index, .. } => {
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index);
            }
            <$event_enum>::Tabled { proposal_index, .. } => {
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index);
            }
            <$event_enum>::Started { ref_index, .. } => {
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index);
            }
            <$event_enum>::Passed { ref_index } => {
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index);
            }
            <$event_enum>::NotPassed { ref_index } => {
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index);
            }
            <$event_enum>::Cancelled { ref_index } => {
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index);
            }
            <$event_enum>::Delegated { who, target } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
                $indexer.index_event_account_id(target, $block_number, $event_index);
            }
            <$event_enum>::Undelegated { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            <$event_enum>::Vetoed {
                who, proposal_hash, ..
            } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                );
            }
            <$event_enum>::Blacklisted { proposal_hash } => {
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                );
            }
            <$event_enum>::Voted {
                voter, ref_index, ..
            } => {
                $indexer.index_event_account_id(voter, $block_number, $event_index);
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index);
            }
            <$event_enum>::Seconded {
                seconder,
                prop_index,
            } => {
                $indexer.index_event_account_id(seconder, $block_number, $event_index);
                $indexer.index_event_proposal_index(prop_index, $block_number, $event_index);
            }
            <$event_enum>::ProposalCanceled { prop_index } => {
                $indexer.index_event_proposal_index(prop_index, $block_number, $event_index);
            }
            <$event_enum>::MetadataSet { owner: _, hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            <$event_enum>::MetadataCleared { owner: _, hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            <$event_enum>::MetadataTransferred {
                prev_owner: _,
                owner: _,
                hash,
            } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            _ => {}
        }
    };
}
