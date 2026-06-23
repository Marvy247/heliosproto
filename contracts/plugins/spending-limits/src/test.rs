use super::{SpendingLimits, SpendingLimitsClient};
use heliosproto_session_keys::{CallContext, PolicyResult};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{symbol_short, vec, Address, Env, IntoVal, Val, Vec};

fn setup(env: &Env) -> (SpendingLimitsClient<'_>, Address) {
    env.mock_all_auths();
    let contract_id = env.register(SpendingLimits, ());
    let client = SpendingLimitsClient::new(env, &contract_id);
    let account = Address::generate(env);
    (client, account)
}

fn transfer_args(env: &Env, from: &Address, to: &Address, amount: i128) -> Vec<Val> {
    vec![env, from.to_val(), to.to_val(), amount.into_val(env)]
}

#[test]
fn no_limit_set_returns_defer() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let token = Address::generate(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    let call = CallContext {
        contract: token,
        function: symbol_short!("transfer"),
        args: transfer_args(&env, &from, &to, 100),
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Defer);
}

#[test]
fn under_limit_allows_and_decreases_remaining() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let token = Address::generate(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    client.set_limit(&account, &token, &1000, &86400);

    let call = CallContext {
        contract: token.clone(),
        function: symbol_short!("transfer"),
        args: transfer_args(&env, &from, &to, 300),
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Allow);
    assert_eq!(client.get_remaining(&token), 700);
}

#[test]
fn at_limit_allows_remaining_zero() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let token = Address::generate(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    client.set_limit(&account, &token, &100, &86400);

    let call = CallContext {
        contract: token.clone(),
        function: symbol_short!("transfer"),
        args: transfer_args(&env, &from, &to, 100),
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Allow);
    assert_eq!(client.get_remaining(&token), 0);
}

#[test]
fn over_limit_denies_remaining_unchanged() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let token = Address::generate(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    client.set_limit(&account, &token, &100, &86400);

    let call = CallContext {
        contract: token.clone(),
        function: symbol_short!("transfer"),
        args: transfer_args(&env, &from, &to, 150),
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Deny);
    assert_eq!(client.get_remaining(&token), 100);
}

#[test]
fn multiple_tokens_have_independent_limits() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let token_a = Address::generate(&env);
    let token_b = Address::generate(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    client.set_limit(&account, &token_a, &500, &86400);
    client.set_limit(&account, &token_b, &200, &86400);

    let call_a = CallContext {
        contract: token_a.clone(),
        function: symbol_short!("transfer"),
        args: transfer_args(&env, &from, &to, 300),
    };
    let call_b = CallContext {
        contract: token_b.clone(),
        function: symbol_short!("transfer"),
        args: transfer_args(&env, &from, &to, 100),
    };

    assert_eq!(client.check(&account, &call_a), PolicyResult::Allow);
    assert_eq!(client.get_remaining(&token_a), 200);
    assert_eq!(client.check(&account, &call_b), PolicyResult::Allow);
    assert_eq!(client.get_remaining(&token_b), 100);
}

#[test]
fn remove_limit_returns_defer() {
    let env = Env::default();
    let (client, account) = setup(&env);

    let token = Address::generate(&env);
    client.set_limit(&account, &token, &100, &86400);
    client.remove_limit(&account, &token);

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let call = CallContext {
        contract: token,
        function: symbol_short!("transfer"),
        args: transfer_args(&env, &from, &to, 50),
    };

    assert_eq!(client.check(&account, &call), PolicyResult::Defer);
}
