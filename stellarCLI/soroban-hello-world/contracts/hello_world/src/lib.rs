#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};
// All contracts hould begin with #![no_std] to ensure that 
// the Rust standard library is not included in the build. 
// The Rust standard library is large and not well suited 
// to being deployed into small programs like those deployed to blockchains.
// required thing are imported from soroban_sdk



#[contract]
pub struct HelloContract;
// list contract functions


// within implemntation block the contract functions 
// are implemented
#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

// this makes Rust to compile and run test code 
// refer to test.rs for this
mod test;
