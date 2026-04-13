#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_create_and_join_game() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ChessellarContract);
    let client = ChessellarContractClient::new(&env, &contract_id);

    let player_white = Address::generate(&env);
    let player_black = Address::generate(&env);

    // Initial creation logic will go here in the next commit
}
