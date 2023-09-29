#[macro_export]
macro_rules! index_system_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::NewAccount { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            <$event_enum>::KilledAccount { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
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
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index)?;
            }
            <$event_enum>::Requested { hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index)?;
            }
            <$event_enum>::Cleared { hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index)?;
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
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
                $indexer.index_event_account_index(index, $block_number, $event_index)?;
            }
            <$event_enum>::IndexFreed { index } => {
                $indexer.index_event_account_index(index, $block_number, $event_index)?;
            }
            <$event_enum>::IndexFrozen { index, who } => {
                $indexer.index_event_account_index(index, $block_number, $event_index)?;
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
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
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            <$event_enum>::DustLost { account, .. } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            <$event_enum>::Transfer { from, to, .. } => {
                $indexer.index_event_account_id(from, $block_number, $event_index)?;
                $indexer.index_event_account_id(to, $block_number, $event_index)?;
            }
            <$event_enum>::BalanceSet { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Reserved { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Unreserved { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::ReserveRepatriated { from, to, .. } => {
                $indexer.index_event_account_id(from, $block_number, $event_index)?;
                $indexer.index_event_account_id(to, $block_number, $event_index)?;
            }
            <$event_enum>::Deposit { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Withdraw { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Slashed { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Minted { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Burned { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Suspended { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Restored { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Upgraded { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Locked { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Unlocked { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Frozen { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Thawed { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
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
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
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
                $indexer.index_event_era_index(era_index, $block_number, $event_index)?;
            }
            <$event_enum>::Rewarded { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
            }
            <$event_enum>::Slashed { staker, .. } => {
                $indexer.index_event_account_id(staker, $block_number, $event_index)?;
            }
            <$event_enum>::SlashReported {
                validator,
                slash_era,
                ..
            } => {
                $indexer.index_event_account_id(validator, $block_number, $event_index)?;
                $indexer.index_event_era_index(slash_era, $block_number, $event_index)?;
            }
            <$event_enum>::OldSlashingReportDiscarded { session_index } => {
                $indexer.index_event_session_index(session_index, $block_number, $event_index)?;
            }
            <$event_enum>::Bonded { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
            }
            <$event_enum>::Unbonded { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
            }
            <$event_enum>::Withdrawn { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
            }
            <$event_enum>::Kicked { nominator, stash } => {
                $indexer.index_event_account_id(nominator, $block_number, $event_index)?;
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
            }
            <$event_enum>::Chilled { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
            }
            <$event_enum>::PayoutStarted {
                era_index,
                validator_stash,
            } => {
                $indexer.index_event_era_index(era_index, $block_number, $event_index)?;
                $indexer.index_event_account_id(validator_stash, $block_number, $event_index)?;
            }
            <$event_enum>::ValidatorPrefsSet { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
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
                $indexer.index_event_session_index(session_index, $block_number, $event_index)?;
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
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index)?;
            }
            <$event_enum>::Tabled { proposal_index, .. } => {
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index)?;
            }
            <$event_enum>::Started { ref_index, .. } => {
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index)?;
            }
            <$event_enum>::Passed { ref_index } => {
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index)?;
            }
            <$event_enum>::NotPassed { ref_index } => {
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index)?;
            }
            <$event_enum>::Cancelled { ref_index } => {
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index)?;
            }
            <$event_enum>::Delegated { who, target } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
                $indexer.index_event_account_id(target, $block_number, $event_index)?;
            }
            <$event_enum>::Undelegated { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            <$event_enum>::Vetoed {
                who, proposal_hash, ..
            } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::Blacklisted { proposal_hash } => {
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::Voted {
                voter, ref_index, ..
            } => {
                $indexer.index_event_account_id(voter, $block_number, $event_index)?;
                $indexer.index_event_ref_index(ref_index, $block_number, $event_index)?;
            }
            <$event_enum>::Seconded {
                seconder,
                prop_index,
            } => {
                $indexer.index_event_account_id(seconder, $block_number, $event_index)?;
                $indexer.index_event_proposal_index(prop_index, $block_number, $event_index)?;
            }
            <$event_enum>::ProposalCanceled { prop_index } => {
                $indexer.index_event_proposal_index(prop_index, $block_number, $event_index)?;
            }
            <$event_enum>::MetadataSet { hash, .. } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index)?;
            }
            <$event_enum>::MetadataCleared { hash, .. } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index)?;
            }
            <$event_enum>::MetadataTransferred { hash, .. } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_collective_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Proposed {
                account,
                proposal_index,
                proposal_hash,
                ..
            } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index)?;
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::Voted {
                account,
                proposal_hash,
                ..
            } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::Approved { proposal_hash } => {
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::Disapproved { proposal_hash } => {
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::Executed { proposal_hash, .. } => {
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::MemberExecuted { proposal_hash, .. } => {
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::Closed { proposal_hash, .. } => {
                $indexer.index_event_proposal_hash(
                    proposal_hash.into(),
                    $block_number,
                    $event_index,
                )?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_elections_phragmen_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::NewTerm { new_members } => {
                for member in &new_members {
                    $indexer.index_event_account_id(
                        member.0.clone(),
                        $block_number,
                        $event_index,
                    )?;
                }
            }
            <$event_enum>::MemberKicked { member } => {
                $indexer.index_event_account_id(member, $block_number, $event_index)?;
            }
            <$event_enum>::Renounced { candidate } => {
                $indexer.index_event_account_id(candidate, $block_number, $event_index)?;
            }
            <$event_enum>::CandidateSlashed { candidate, .. } => {
                $indexer.index_event_account_id(candidate, $block_number, $event_index)?;
            }
            <$event_enum>::SeatHolderSlashed { seat_holder, .. } => {
                $indexer.index_event_account_id(seat_holder, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_treasury_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Proposed { proposal_index } => {
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index)?;
            }
            <$event_enum>::Awarded {
                proposal_index,
                account,
                ..
            } => {
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index)?;
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            <$event_enum>::Rejected { proposal_index, .. } => {
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index)?;
            }
            <$event_enum>::SpendApproved {
                proposal_index,
                beneficiary,
                ..
            } => {
                $indexer.index_event_proposal_index(proposal_index, $block_number, $event_index)?;
                $indexer.index_event_account_id(beneficiary, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_vesting_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::VestingUpdated { account, .. } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            <$event_enum>::VestingCompleted { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_identity_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::IdentitySet { who } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::IdentityCleared { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::IdentityKilled { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::JudgementRequested {
                who,
                registrar_index,
                ..
            } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
                $indexer.index_event_registrar_index(
                    registrar_index,
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::JudgementUnrequested {
                who,
                registrar_index,
                ..
            } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
                $indexer.index_event_registrar_index(
                    registrar_index,
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::JudgementGiven {
                target,
                registrar_index,
                ..
            } => {
                $indexer.index_event_account_id(target, $block_number, $event_index)?;
                $indexer.index_event_registrar_index(
                    registrar_index,
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::RegistrarAdded {
                registrar_index, ..
            } => {
                $indexer.index_event_registrar_index(
                    registrar_index,
                    $block_number,
                    $event_index,
                )?;
            }
            <$event_enum>::SubIdentityAdded { sub, main, .. } => {
                $indexer.index_event_account_id(sub, $block_number, $event_index)?;
                $indexer.index_event_account_id(main, $block_number, $event_index)?;
            }
            <$event_enum>::SubIdentityRemoved { sub, main, .. } => {
                $indexer.index_event_account_id(sub, $block_number, $event_index)?;
                $indexer.index_event_account_id(main, $block_number, $event_index)?;
            }
            <$event_enum>::SubIdentityRevoked { sub, main, .. } => {
                $indexer.index_event_account_id(sub, $block_number, $event_index)?;
                $indexer.index_event_account_id(main, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_proxy_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::PureCreated { pure, who, .. } => {
                $indexer.index_event_account_id(pure, $block_number, $event_index)?;
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::Announced { real, proxy, .. } => {
                $indexer.index_event_account_id(real, $block_number, $event_index)?;
                $indexer.index_event_account_id(proxy, $block_number, $event_index)?;
            }
            <$event_enum>::ProxyAdded {
                delegator,
                delegatee,
                ..
            } => {
                $indexer.index_event_account_id(delegator, $block_number, $event_index)?;
                $indexer.index_event_account_id(delegatee, $block_number, $event_index)?;
            }
            <$event_enum>::ProxyRemoved {
                delegator,
                delegatee,
                ..
            } => {
                $indexer.index_event_account_id(delegator, $block_number, $event_index)?;
                $indexer.index_event_account_id(delegatee, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_multisig_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::NewMultisig {
                approving,
                multisig,
                ..
            } => {
                $indexer.index_event_account_id(approving, $block_number, $event_index)?;
                $indexer.index_event_account_id(multisig, $block_number, $event_index)?;
            }
            <$event_enum>::MultisigApproval {
                approving,
                multisig,
                ..
            } => {
                $indexer.index_event_account_id(approving, $block_number, $event_index)?;
                $indexer.index_event_account_id(multisig, $block_number, $event_index)?;
            }
            <$event_enum>::MultisigExecuted {
                approving,
                multisig,
                ..
            } => {
                $indexer.index_event_account_id(approving, $block_number, $event_index)?;
                $indexer.index_event_account_id(multisig, $block_number, $event_index)?;
            }
            <$event_enum>::MultisigCancelled {
                cancelling,
                multisig,
                ..
            } => {
                $indexer.index_event_account_id(cancelling, $block_number, $event_index)?;
                $indexer.index_event_account_id(multisig, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_fast_unstake_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Unstaked { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
            }
            <$event_enum>::Slashed { stash, .. } => {
                $indexer.index_event_account_id(stash, $block_number, $event_index)?;
            }
            <$event_enum>::BatchChecked { eras } => {
                for era in eras {
                    $indexer.index_event_era_index(era, $block_number, $event_index)?;
                }
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_election_provider_multi_phase_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::SolutionStored { origin, .. } => {
                if let Some(account) = origin {
                    $indexer.index_event_account_id(account, $block_number, $event_index)?;
                }
            }
            <$event_enum>::Rewarded { account, .. } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            <$event_enum>::Slashed { account, .. } => {
                $indexer.index_event_account_id(account, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_tips_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::NewTip { tip_hash } => {
                $indexer.index_event_tip_hash(tip_hash.into(), $block_number, $event_index)?;
            }
            <$event_enum>::TipClosing { tip_hash } => {
                $indexer.index_event_tip_hash(tip_hash.into(), $block_number, $event_index)?;
            }
            <$event_enum>::TipClosed { tip_hash, who, .. } => {
                $indexer.index_event_tip_hash(tip_hash.into(), $block_number, $event_index)?;
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::TipRetracted { tip_hash } => {
                $indexer.index_event_tip_hash(tip_hash.into(), $block_number, $event_index)?;
            }
            <$event_enum>::TipSlashed {
                tip_hash, finder, ..
            } => {
                $indexer.index_event_tip_hash(tip_hash.into(), $block_number, $event_index)?;
                $indexer.index_event_account_id(finder, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_bounties_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::BountyProposed { index } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
            }
            <$event_enum>::BountyRejected { index, .. } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
            }
            <$event_enum>::BountyBecameActive { index } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
            }
            <$event_enum>::BountyAwarded { index, beneficiary } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
                $indexer.index_event_account_id(beneficiary, $block_number, $event_index)?;
            }
            <$event_enum>::BountyClaimed {
                index, beneficiary, ..
            } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
                $indexer.index_event_account_id(beneficiary, $block_number, $event_index)?;
            }
            <$event_enum>::BountyCanceled { index } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
            }
            <$event_enum>::BountyExtended { index } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_child_bounties_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Added { index, child_index } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
                $indexer.index_event_bounty_index(child_index, $block_number, $event_index)?;
            }
            <$event_enum>::Awarded {
                index,
                child_index,
                beneficiary,
            } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
                $indexer.index_event_bounty_index(child_index, $block_number, $event_index)?;
                $indexer.index_event_account_id(beneficiary, $block_number, $event_index)?;
            }
            <$event_enum>::Claimed {
                index,
                child_index,
                beneficiary,
                ..
            } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
                $indexer.index_event_bounty_index(child_index, $block_number, $event_index)?;
                $indexer.index_event_account_id(beneficiary, $block_number, $event_index)?;
            }
            <$event_enum>::Canceled { index, child_index } => {
                $indexer.index_event_bounty_index(index, $block_number, $event_index)?;
                $indexer.index_event_bounty_index(child_index, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_bags_list_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Rebagged { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            <$event_enum>::ScoreUpdated { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_nomination_pools_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Created { depositor, pool_id } => {
                $indexer.index_event_account_id(depositor, $block_number, $event_index)?;
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::Bonded {
                member, pool_id, ..
            } => {
                $indexer.index_event_account_id(member, $block_number, $event_index)?;
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::PaidOut {
                member, pool_id, ..
            } => {
                $indexer.index_event_account_id(member, $block_number, $event_index)?;
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::Unbonded {
                member,
                pool_id,
                era,
                ..
            } => {
                $indexer.index_event_account_id(member, $block_number, $event_index)?;
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
                $indexer.index_event_era_index(era, $block_number, $event_index)?;
            }
            <$event_enum>::Withdrawn {
                member, pool_id, ..
            } => {
                $indexer.index_event_account_id(member, $block_number, $event_index)?;
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::Destroyed { pool_id } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::StateChanged { pool_id, .. } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::MemberRemoved { pool_id, member } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
                $indexer.index_event_account_id(member, $block_number, $event_index)?;
            }
            <$event_enum>::RolesUpdated {
                root,
                bouncer,
                nominator,
            } => {
                if let Some(account) = root {
                    $indexer.index_event_account_id(account, $block_number, $event_index)?;
                }
                if let Some(account) = bouncer {
                    $indexer.index_event_account_id(account, $block_number, $event_index)?;
                }
                if let Some(account) = nominator {
                    $indexer.index_event_account_id(account, $block_number, $event_index)?;
                }
            }
            <$event_enum>::PoolSlashed { pool_id, .. } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::UnbondingPoolSlashed { pool_id, era, .. } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
                $indexer.index_event_era_index(era, $block_number, $event_index)?;
            }
            <$event_enum>::PoolCommissionUpdated {
                pool_id, current, ..
            } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
                if let Some((i, account)) = current {
                    $indexer.index_event_account_id(account, $block_number, $event_index)?;
                }
            }
            <$event_enum>::PoolMaxCommissionUpdated { pool_id, .. } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::PoolCommissionChangeRateUpdated { pool_id, .. } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::PoolCommissionChangeRateUpdated { pool_id, .. } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            <$event_enum>::PoolCommissionClaimed { pool_id, .. } => {
                $indexer.index_event_pool_id(pool_id, $block_number, $event_index)?;
            }
            _ => {}
        }
    };
}
