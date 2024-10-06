#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};
// env is required for any test
// soroban environment that contract run inside 
// is env


#[test]
fn test() {
    let env = Env::default();
    // making id parameter none causes it to generate one 
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Dev"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Dev"),
        ]
    );
}
