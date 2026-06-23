#![no_std]

use heliosproto_session_keys::{CallContext, PolicyResult};
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, TryFromVal};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LimitInfo {
    pub cap: i128,
    pub window_seconds: u64,
    pub window_start: u64,
    pub remaining: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LimitKey {
    pub token: Address,
}

fn get_limit(env: &Env, token: &Address) -> Option<LimitInfo> {
    let key = LimitKey {
        token: token.clone(),
    };
    env.storage().instance().get(&key)
}

fn set_limit_internal(env: &Env, token: &Address, info: &LimitInfo) {
    let key = LimitKey {
        token: token.clone(),
    };
    env.storage().instance().set(&key, info);
}

fn remove_limit_internal(env: &Env, token: &Address) {
    let key = LimitKey {
        token: token.clone(),
    };
    env.storage().instance().remove(&key);
}

#[contract]
pub struct SpendingLimits;

#[contractimpl]
impl SpendingLimits {
    pub fn set_limit(env: Env, account: Address, token: Address, cap: i128, window_seconds: u64) {
        account.require_auth();
        let now = env.ledger().timestamp();
        let info = LimitInfo {
            cap,
            window_seconds,
            window_start: now,
            remaining: cap,
        };
        set_limit_internal(&env, &token, &info);
    }

    pub fn remove_limit(env: Env, account: Address, token: Address) {
        account.require_auth();
        remove_limit_internal(&env, &token);
    }

    pub fn get_remaining(env: Env, token: Address) -> i128 {
        match get_limit(&env, &token) {
            Some(info) => {
                let now = env.ledger().timestamp();
                if now >= info.window_start + info.window_seconds {
                    info.cap
                } else {
                    info.remaining
                }
            }
            None => 0,
        }
    }

    pub fn check(env: Env, account: Address, call: CallContext) -> PolicyResult {
        _ = account;
        let token = call.contract;

        match get_limit(&env, &token) {
            None => PolicyResult::Defer,
            Some(mut info) => {
                let now = env.ledger().timestamp();

                if now >= info.window_start + info.window_seconds {
                    info.window_start = now;
                    info.remaining = info.cap;
                }

                let amount: i128 = if call.args.len() >= 3 {
                    let val = call.args.get(2).unwrap();
                    i128::try_from_val(&env, &val).unwrap_or(0)
                } else {
                    0
                };

                if amount > info.remaining {
                    return PolicyResult::Deny;
                }

                info.remaining -= amount;
                set_limit_internal(&env, &token, &info);
                PolicyResult::Allow
            }
        }
    }
}

#[cfg(test)]
mod test;
