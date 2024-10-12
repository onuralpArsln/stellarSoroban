#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec, log, symbol_short, Symbol};


#[contract]
pub struct FrontendContract;

#[contractimpl]
impl FrontendContract {
       pub fn greet(env: Env, name: Symbol) -> Vec<Symbol> {
        // Create the prefix symbol
           let prefix = Symbol::new(&env, "Welcome");
        
        // Create a vector of Symbols
        vec![&env, prefix, name]
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = env.register_contract(None, FrontendContract);
        let client = FrontendContractClient::new(&env, &contract_id);

        let name = Symbol::new(&env, "Developer");
        let result = client.greet(&name);
        assert_eq!(result, Symbol::new(&env, "Welcome_Developer"));
    }
}