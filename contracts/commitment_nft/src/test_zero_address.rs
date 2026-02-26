#![cfg(test)]
extern crate std;

use soroban_sdk::{Address, Env, String};
use crate::{CommitmentNftContract, CommitmentNftContractClient};

fn generate_zero_address(env: &Env) -> Address {
    Address::from_string(&String::from_str(env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"))
}

#[test]
#[should_panic] 
fn test_nft_mint_to_zero_address_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CommitmentNftContract);
    let client = CommitmentNftContractClient::new(&env, &contract_id);

    let zero_address = generate_zero_address(&env);

    // Standard mint call
    client.mint(&zero_address); 
}

#[test]
#[should_panic] 
fn test_nft_transfer_to_zero_address_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CommitmentNftContract);
    let client = CommitmentNftContractClient::new(&env, &contract_id);

    let sender = Address::generate(&env);
    let zero_address = generate_zero_address(&env);

    // Setup: Mint to valid sender
    client.mint(&sender);
    
    // Use a standard i128 token_id
    let token_id: i128 = 0; 

    client.transfer(&sender, &zero_address, &token_id);
}