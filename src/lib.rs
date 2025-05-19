#![no_std]

use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};

#[contract]
pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    pub fn swap(
        env: Env,
        a: Address,
        b: Address,
        token_a: Address,
        token_b: Address,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) {
        if amount_b < min_b_for_a {
            panic!("amount_b is less than min_b_for_a");
        }
        if amount_a < min_a_for_b {
            panic!("amount_a is less than min_a_for_b");
        }

        a.require_auth_for_args(
            (token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(&env),
        );
        b.require_auth_for_args(
            (token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(&env),
        );

        move_token_internal(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        move_token_internal(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
}

fn move_token_internal(
    env: &Env,
    token_address: &Address,
    from_party: &Address,
    to_party: &Address,
    committed_amount: i128, // Party 'from_party' commits this much to the contract
    actual_transfer_amount: i128, // Party 'to_party' will receive this much
) {
    let token_client = token::Client::new(env, token_address);
    let contract_id = env.current_contract_address();

    token_client.transfer(from_party, &contract_id, &committed_amount);
    token_client.transfer(&contract_id, to_party, &actual_transfer_amount);

    let refund_amount = committed_amount - actual_transfer_amount;
    if refund_amount > 0 {
        token_client.transfer(&contract_id, from_party, &refund_amount);
    }
}

#[cfg(test)]
mod test;