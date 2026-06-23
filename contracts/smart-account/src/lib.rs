#![no_std]

use soroban_sdk::auth::{Context, CustomAccountInterface};
use soroban_sdk::crypto::Hash;
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, BytesN, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignerKind {
    Ed25519,
    Secp256r1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signer {
    pub kind: SignerKind,
    pub key: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Signers,
    Threshold,
    Nonce,
    Plugins,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitArgs {
    pub signers: Vec<Signer>,
    pub threshold: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthPayload {
    pub nonce: u64,
    pub pubkeys: Vec<BytesN<32>>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AuthError {
    BadNonce = 1,
    InvalidSignature = 2,
    InsufficientWeight = 3,
}

#[contract]
pub struct SmartAccount;

#[contractimpl]
impl CustomAccountInterface for SmartAccount {
    type Signature = AuthPayload;
    type Error = AuthError;

    fn __check_auth(
        env: Env,
        _signature_payload: Hash<32>,
        signatures: Self::Signature,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Self::Error> {
        let stored_nonce: u64 = env.storage().instance().get(&DataKey::Nonce).unwrap_or(0);
        if signatures.nonce != stored_nonce {
            return Err(AuthError::BadNonce);
        }

        let stored_signers: Vec<Signer> = env
            .storage()
            .instance()
            .get(&DataKey::Signers)
            .expect("not initialized");
        let threshold: u32 = env
            .storage()
            .instance()
            .get(&DataKey::Threshold)
            .expect("not initialized");

        let mut valid_count: u32 = 0;
        for pubkey in signatures.pubkeys.iter() {
            let found = stored_signers.iter().any(|signer| signer.key == pubkey);
            if found {
                valid_count += 1;
            }
        }

        if valid_count < threshold {
            return Err(AuthError::InvalidSignature);
        }

        env.storage()
            .instance()
            .set(&DataKey::Nonce, &(stored_nonce + 1));
        Ok(())
    }
}

#[contractimpl]
impl SmartAccount {
    pub fn init(env: Env, args: InitArgs) {
        if env.storage().instance().has(&DataKey::Signers) {
            panic!("already initialized");
        }
        if args.signers.is_empty() {
            panic!("at least one signer required");
        }
        if args.threshold == 0 || args.threshold > args.signers.len() {
            panic!("invalid threshold");
        }

        env.storage()
            .instance()
            .set(&DataKey::Signers, &args.signers);
        env.storage()
            .instance()
            .set(&DataKey::Threshold, &args.threshold);
        env.storage().instance().set(&DataKey::Nonce, &0u64);
    }

    pub fn signers(env: Env) -> Vec<Signer> {
        env.storage()
            .instance()
            .get(&DataKey::Signers)
            .expect("not initialized")
    }

    pub fn threshold(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Threshold)
            .expect("not initialized")
    }

    pub fn nonce(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::Nonce).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
