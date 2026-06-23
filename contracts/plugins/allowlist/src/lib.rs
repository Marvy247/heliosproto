#![no_std]

use heliosproto_session_keys::{CallContext, PolicyResult};
use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Allowlist,
}

#[contract]
pub struct Allowlist;

#[contractimpl]
impl Allowlist {
    /// Add a contract address to the allowlist for the given account.
    pub fn add(env: Env, account: Address, addr: Address) {
        account.require_auth();
        let mut list: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Allowlist)
            .unwrap_or(vec![&env]);

        if !list.contains(&addr) {
            list.push_back(addr);
        }

        env.storage().instance().set(&DataKey::Allowlist, &list);
    }

    /// Remove a contract address from the allowlist for the given account.
    pub fn remove(env: Env, account: Address, addr: Address) {
        account.require_auth();
        let list: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Allowlist)
            .unwrap_or(vec![&env]);

        let mut filtered: Vec<Address> = vec![&env];
        for a in list.iter() {
            if a != addr {
                filtered.push_back(a);
            }
        }

        env.storage().instance().set(&DataKey::Allowlist, &filtered);
    }

    /// List all allowed contract addresses.
    pub fn list(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Allowlist)
            .unwrap_or(vec![&env])
    }

    /// Check if a call's destination is allowed.
    pub fn check(env: Env, account: Address, call: CallContext) -> PolicyResult {
        _ = account;
        let list: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Allowlist)
            .unwrap_or(vec![&env]);

        if list.contains(&call.contract) {
            PolicyResult::Allow
        } else {
            PolicyResult::Deny
        }
    }
}

#[cfg(test)]
mod test;
