#![cfg(test)]
extern crate std;

use soroban_sdk::{Address, Env, String};
use crate::{CommitmentCoreContract, CommitmentCoreContractClient, CommitmentRules};

fn generate_zero_address(env: &Env) -> Address {
    Address::from_string(&String::from_str(env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"))
}

#[test]
#[should_panic] 
fn test_create_commitment_zero_owner_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CommitmentCoreContract);
    let client = CommitmentCoreContractClient::new(&env, &contract_id);

    let zero_owner = generate_zero_address(&env);
    let amount: i128 = 1000;
    let asset_address = Address::generate(&env);
    
    // Manually initialize the struct to avoid E0599 "method not found" errors
    // If your struct has different fields, the compiler will tell us exactly which ones to add/change.
    let rules = CommitmentRules {
        min_amount: 0,
        max_amount: 1000000,
        min_duration: 0,
        max_duration: 1000000,
    }; 

    client.create_commitment(&zero_owner, &amount, &asset_address, &rules);
}