use super::{
    AuthError, AuthPayload, InitArgs, Signer, SignerKind, SmartAccount, SmartAccountClient,
};
use soroban_sdk::{vec, BytesN, Env, IntoVal, Vec};

fn key(env: &Env, byte: u8) -> BytesN<32> {
    let mut arr = [0u8; 32];
    arr[0] = byte;
    BytesN::from_array(env, &arr)
}

fn ed25519_signer(env: &Env, byte: u8) -> Signer {
    Signer {
        kind: SignerKind::Ed25519,
        key: key(env, byte),
    }
}

fn setup_with_signers(env: &Env, signers: Vec<Signer>, threshold: u32) -> SmartAccountClient<'_> {
    let contract_id = env.register(SmartAccount, ());
    let client = SmartAccountClient::new(env, &contract_id);
    client.init(&InitArgs { signers, threshold });
    client
}

#[test]
fn init_with_single_signer_sets_threshold_one() {
    let env = Env::default();
    let client = setup_with_signers(&env, vec![&env, ed25519_signer(&env, 1)], 1);

    assert_eq!(client.signers(), vec![&env, ed25519_signer(&env, 1)]);
    assert_eq!(client.threshold(), 1);
    assert_eq!(client.nonce(), 0);
}

#[test]
fn init_with_multisig_two_of_three() {
    let env = Env::default();
    let signers = vec![
        &env,
        ed25519_signer(&env, 1),
        ed25519_signer(&env, 2),
        ed25519_signer(&env, 3),
    ];
    let client = setup_with_signers(&env, signers.clone(), 2);

    assert_eq!(client.signers().len(), 3);
    assert_eq!(client.threshold(), 2);
}

#[test]
#[should_panic(expected = "already initialized")]
fn init_twice_panics() {
    let env = Env::default();
    let client = setup_with_signers(&env, vec![&env, ed25519_signer(&env, 1)], 1);
    client.init(&InitArgs {
        signers: vec![&env, ed25519_signer(&env, 1)],
        threshold: 1,
    });
}

#[test]
#[should_panic(expected = "invalid threshold")]
fn threshold_greater_than_signers_panics() {
    let env = Env::default();
    let _client = setup_with_signers(&env, vec![&env, ed25519_signer(&env, 1)], 5);
}

#[test]
#[should_panic(expected = "at least one signer required")]
fn empty_signers_panics() {
    let env = Env::default();
    let _client = setup_with_signers(&env, vec![&env], 1);
}

#[test]
fn valid_single_signer_correct_nonce_passes() {
    let env = Env::default();
    let signers = vec![&env, ed25519_signer(&env, 1)];
    let client = setup_with_signers(&env, signers, 1);

    let payload = AuthPayload {
        nonce: 0,
        pubkeys: vec![&env, key(&env, 1)],
    };
    let res: Result<(), Result<AuthError, soroban_sdk::InvokeError>> = env
        .try_invoke_contract_check_auth::<AuthError>(
            &client.address,
            &BytesN::from_array(&env, &[0; 32]),
            payload.into_val(&env),
            &vec![&env],
        );
    assert!(res.is_ok());
    assert_eq!(client.nonce(), 1);
}

#[test]
fn wrong_signature_panics() {
    let env = Env::default();
    let signers = vec![&env, ed25519_signer(&env, 1)];
    let client = setup_with_signers(&env, signers, 1);

    let payload = AuthPayload {
        nonce: 0,
        pubkeys: vec![&env, key(&env, 99)], // not a stored signer
    };
    let res: Result<(), Result<AuthError, soroban_sdk::InvokeError>> = env
        .try_invoke_contract_check_auth::<AuthError>(
            &client.address,
            &BytesN::from_array(&env, &[0; 32]),
            payload.into_val(&env),
            &vec![&env],
        );
    assert!(matches!(res, Err(Ok(AuthError::InvalidSignature))));
}

#[test]
fn stale_nonce_panics() {
    let env = Env::default();
    let signers = vec![&env, ed25519_signer(&env, 1)];
    let client = setup_with_signers(&env, signers, 1);

    let payload = AuthPayload {
        nonce: 42, // wrong nonce
        pubkeys: vec![&env, key(&env, 1)],
    };
    let res: Result<(), Result<AuthError, soroban_sdk::InvokeError>> = env
        .try_invoke_contract_check_auth::<AuthError>(
            &client.address,
            &BytesN::from_array(&env, &[0; 32]),
            payload.into_val(&env),
            &vec![&env],
        );
    assert!(matches!(res, Err(Ok(AuthError::BadNonce))));
}

#[test]
fn multisig_two_of_three_with_one_signature_panics() {
    let env = Env::default();
    let signers = vec![
        &env,
        ed25519_signer(&env, 1),
        ed25519_signer(&env, 2),
        ed25519_signer(&env, 3),
    ];
    let client = setup_with_signers(&env, signers, 2);

    let payload = AuthPayload {
        nonce: 0,
        pubkeys: vec![&env, key(&env, 1)], // only 1 of 2 needed
    };
    let res: Result<(), Result<AuthError, soroban_sdk::InvokeError>> = env
        .try_invoke_contract_check_auth::<AuthError>(
            &client.address,
            &BytesN::from_array(&env, &[0; 32]),
            payload.into_val(&env),
            &vec![&env],
        );
    assert!(matches!(res, Err(Ok(AuthError::InvalidSignature))));
}

#[test]
fn multisig_two_of_three_with_two_signatures_passes() {
    let env = Env::default();
    let signers = vec![
        &env,
        ed25519_signer(&env, 1),
        ed25519_signer(&env, 2),
        ed25519_signer(&env, 3),
    ];
    let client = setup_with_signers(&env, signers, 2);

    let payload = AuthPayload {
        nonce: 0,
        pubkeys: vec![&env, key(&env, 1), key(&env, 2)], // 2 of 3
    };
    let res: Result<(), Result<AuthError, soroban_sdk::InvokeError>> = env
        .try_invoke_contract_check_auth::<AuthError>(
            &client.address,
            &BytesN::from_array(&env, &[0; 32]),
            payload.into_val(&env),
            &vec![&env],
        );
    assert!(res.is_ok());
    assert_eq!(client.nonce(), 1);
}
