use super::{Allowlist, AllowlistClient};
use heliosproto_session_keys::{CallContext, PolicyResult};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{symbol_short, vec, Address, Env};

fn setup(env: &Env) -> (AllowlistClient<'_>, Address) {
    env.mock_all_auths();
    let contract_id = env.register(Allowlist, ());
    let client = AllowlistClient::new(env, &contract_id);
    let account = Address::generate(env);
    (client, account)
}

#[test]
fn empty_allowlist_denies_all() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let call = CallContext {
        contract: Address::generate(&env),
        function: symbol_short!("transfer"),
        args: vec![&env],
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Deny);
}

#[test]
fn add_then_check_allows() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let allowed = Address::generate(&env);
    client.add(&account, &allowed);

    let call = CallContext {
        contract: allowed.clone(),
        function: symbol_short!("transfer"),
        args: vec![&env],
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Allow);
}

#[test]
fn add_two_remove_one_leaves_only_one() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let addr1 = Address::generate(&env);
    let addr2 = Address::generate(&env);

    client.add(&account, &addr1);
    client.add(&account, &addr2);
    client.remove(&account, &addr1);

    let list = client.list();
    assert_eq!(list.len(), 1);
    assert!(list.get(0).unwrap() == addr2);
}

#[test]
fn add_same_address_twice_is_idempotent() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let addr = Address::generate(&env);
    client.add(&account, &addr);
    client.add(&account, &addr);

    let list = client.list();
    assert_eq!(list.len(), 1);
}

#[test]
fn check_allowed_address_returns_allow() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let allowed = Address::generate(&env);
    client.add(&account, &allowed);

    let call = CallContext {
        contract: allowed,
        function: symbol_short!("transfer"),
        args: vec![&env],
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Allow);
}

#[test]
fn check_denied_address_returns_deny() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let allowed = Address::generate(&env);
    let denied = Address::generate(&env);
    client.add(&account, &allowed);

    let call = CallContext {
        contract: denied,
        function: symbol_short!("transfer"),
        args: vec![&env],
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Deny);
}

#[test]
fn list_returns_all_added_addresses() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let addr1 = Address::generate(&env);
    let addr2 = Address::generate(&env);

    client.add(&account, &addr1);
    client.add(&account, &addr2);

    let list = client.list();
    assert_eq!(list.len(), 2);
    assert!(list.contains(&addr1));
    assert!(list.contains(&addr2));
}

#[test]
fn check_returns_deny_for_contract_not_in_list() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let allowed = Address::generate(&env);
    let not_allowed = Address::generate(&env);
    client.add(&account, &allowed);

    let call = CallContext {
        contract: not_allowed,
        function: symbol_short!("transfer"),
        args: vec![&env],
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Deny);
}
