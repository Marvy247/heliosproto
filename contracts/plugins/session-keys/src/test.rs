use super::{CallContext, PolicyResult, SessionKeys, SessionKeysClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{symbol_short, vec, Address, Env};

#[test]
fn check_returns_defer_for_any_call() {
    let env = Env::default();
    let contract_id = env.register(SessionKeys, ());
    let client = SessionKeysClient::new(&env, &contract_id);

    let call = CallContext {
        contract: Address::generate(&env),
        function: symbol_short!("test"),
        args: vec![&env],
    };

    assert_eq!(
        client.check(&Address::generate(&env), &call),
        PolicyResult::Defer,
    );
}
