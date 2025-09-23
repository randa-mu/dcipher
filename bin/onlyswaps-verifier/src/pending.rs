use crate::eth::ChainState;
use crate::util::normalise_chain_id;
use alloy::primitives::FixedBytes;
use anyhow::anyhow;
use omnievent::types::EventFieldData;
use std::collections::HashSet;
use std::hash::Hash;

/// Collects all `ChainState`s, folds them by status,
/// and returns those that still need to be verified.
///
/// - `Verified` takes priority over `Fulfilled`.
///   If a map entry already exists when iterating over fulfilled states,
///   we assume it has been verified on the source chain.
/// - Multiple `Fulfilled` entries are possible if a solver maliciously
///   claims to fulfill the same request on multiple chains.
///   At most one of these will be verified by dcipher.
///
/// # Arguments
/// * `states` - A vector of `ChainState` values to process
///
/// # Returns
/// A vector of `Route`s representing the pending requests to be verified.
pub(crate) fn extract_pending_verifications<ID: Copy + Eq + Hash>(
    states: Vec<ChainState<ID>>,
) -> Vec<Verification<ID>> {
    let all_states: Vec<RouteStatus<ID>> = states
        .into_iter()
        .fold(HashSet::<RouteStatus<ID>>::new(), |mut out, state| {
            for request_id in state.verified {
                // first we delete any existing fulfilled entries, because verification is god
                out.retain(|entry| entry.route.request_id != request_id);
                // then we insert the new route
                out.insert(RouteStatus {
                    route: Verification {
                        chain_id: state.chain_id,
                        request_id,
                    },
                    status: Status::Verified,
                });
            }
            for request_id in state.fulfilled {
                let existing_fulfillment = out
                    .iter()
                    .find(|route_status| route_status.route.request_id == request_id);

                if let Some(route_status) = existing_fulfillment
                    && route_status.status == Status::Verified
                {
                    continue;
                }

                let route = Verification {
                    chain_id: state.chain_id,
                    request_id,
                };
                out.insert(RouteStatus {
                    route,
                    status: Status::Fulfilled,
                });
            }
            out
        })
        .into_iter()
        .collect();

    // we then filter out all the `request_ids` that have already been verified and return the routes
    // that are outstanding

    all_states
        .into_iter()
        .filter_map(|s| match s.status {
            Status::Fulfilled => Some(s.route),
            _ => None,
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RouteStatus<ID: Copy + Eq + Hash> {
    route: Verification<ID>,
    status: Status,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Status {
    Fulfilled,
    Verified,
}
pub type RequestId = FixedBytes<32>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Verification<ID> {
    pub chain_id: u64,
    pub request_id: ID,
}

impl TryFrom<Vec<EventFieldData>> for Verification<FixedBytes<32>> {
    type Error = anyhow::Error;

    fn try_from(value: Vec<EventFieldData>) -> anyhow::Result<Self> {
        if value.len() != 3 {
            anyhow::bail!("not enough fields in event")
        }

        let (request_id, request_id_len) = value[0]
            .data
            .as_fixed_bytes()
            .ok_or(anyhow!("received event with invalid `requestId`"))?;
        if request_id_len != 32 {
            anyhow::bail!(
                "`requestId` had wrong length; expected 32 got {}",
                request_id.len()
            );
        }

        let (dest_chain_id, _) = value[2]
            .data
            .as_uint()
            .ok_or(anyhow!("received event with invalid `dest_chain_id`"))?;

        Ok(Verification {
            chain_id: normalise_chain_id(dest_chain_id),
            request_id: FixedBytes(request_id.try_into()?),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::eth::ChainState;
    use crate::pending::{Verification, extract_pending_verifications};
    use speculoos::assert_that;
    use speculoos::iter::ContainingIntoIterAssertions;

    #[test]
    fn given_fulfilled_and_not_verified_return_it() {
        let states = vec![ChainState {
            chain_id: 1,
            fulfilled: vec![1],
            verified: vec![],
        }];

        let pending = extract_pending_verifications(states);

        assert_eq!(
            pending,
            vec![Verification {
                chain_id: 1,
                request_id: 1
            }]
        );
    }

    #[test]
    fn given_multiple_chains_we_get_routes_for_each() {
        let states = vec![
            ChainState {
                chain_id: 1,
                fulfilled: vec![1],
                verified: vec![],
            },
            ChainState {
                chain_id: 2,
                fulfilled: vec![2],
                verified: vec![],
            },
        ];

        let pending = extract_pending_verifications(states);

        assert_that!(pending).contains_all_of(&vec![
            &Verification {
                chain_id: 1,
                request_id: 1,
            },
            &Verification {
                chain_id: 2,
                request_id: 2,
            },
        ]);
    }

    #[test]
    fn given_verified_already_fulfilled_not_returned() {
        let states = vec![
            ChainState {
                chain_id: 1,
                fulfilled: vec![1],
                verified: vec![],
            },
            ChainState {
                chain_id: 2,
                fulfilled: vec![],
                verified: vec![1],
            },
        ];

        let pending = extract_pending_verifications(states);

        assert_eq!(pending, vec![]);
    }

    #[test]
    fn given_verified_comes_first_fulfilled_still_filtered() {
        let states = vec![
            ChainState {
                chain_id: 1,
                fulfilled: vec![],
                verified: vec![1],
            },
            ChainState {
                chain_id: 2,
                fulfilled: vec![1],
                verified: vec![],
            },
        ];

        let pending = extract_pending_verifications(states);

        assert_eq!(pending, vec![]);
    }

    #[test]
    fn duplicate_request_ids_output_multiple_routes() {
        // honestly this shouldn't happen, but perhaps a malicious solver
        // could try it on the wrong chains to confuse dcipher nodes
        let states = vec![
            ChainState {
                chain_id: 1,
                fulfilled: vec![1],
                verified: vec![7, 8, 9],
            },
            ChainState {
                chain_id: 2,
                fulfilled: vec![1],
                verified: vec![],
            },
        ];

        let pending = extract_pending_verifications(states);

        assert_that!(pending).contains_all_of(&vec![
            &Verification {
                chain_id: 1,
                request_id: 1,
            },
            &Verification {
                chain_id: 2,
                request_id: 1,
            },
        ]);
    }

    #[test]
    fn mix_of_verified_and_fulfilled_returns_expected() {
        let states = vec![
            ChainState {
                chain_id: 1,
                fulfilled: vec![1, 2, 3],
                verified: vec![7, 8, 9, 4],
            },
            ChainState {
                chain_id: 2,
                fulfilled: vec![3],
                verified: vec![6],
            },
            ChainState {
                chain_id: 3,
                fulfilled: vec![4, 5, 6],
                verified: vec![1],
            },
        ];

        let pending = extract_pending_verifications(states);

        assert_that!(pending).contains_all_of(&vec![
            &Verification {
                chain_id: 1,
                request_id: 2,
            },
            &Verification {
                chain_id: 1,
                request_id: 3,
            },
            &Verification {
                chain_id: 2,
                request_id: 3,
            },
            &Verification {
                chain_id: 3,
                request_id: 5,
            },
        ]);
    }
}
