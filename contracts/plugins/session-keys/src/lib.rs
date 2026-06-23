#![no_std]

//! Session-keys plugin for Helios Protocol smart accounts.
//!
//! This crate defines the shared `PolicyResult` enum and `CallContext` struct
//! that all policy plugins use. The session-keys plugin itself will be
//! implemented in a follow-up issue.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Val, Vec};

/// The result of a plugin policy check.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PolicyResult {
    /// The action is explicitly permitted.
    Allow,
    /// The action is explicitly denied.
    Deny,
    /// The plugin has no opinion; defer to other plugins.
    Defer,
}

/// The context of a single outbound call the plugin must evaluate.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallContext {
    /// The destination contract address being called.
    pub contract: Address,
    /// The function name being invoked.
    pub function: soroban_sdk::Symbol,
    /// The raw arguments to the function.
    pub args: Vec<Val>,
}

#[contract]
pub struct SessionKeys;

#[contractimpl]
impl SessionKeys {
    /// Placeholder entry point. Full session-keys implementation in a follow-up.
    pub fn check(env: Env, account: Address, call: CallContext) -> PolicyResult {
        _ = (env, account, call);
        PolicyResult::Defer
    }
}

#[cfg(test)]
mod test;
