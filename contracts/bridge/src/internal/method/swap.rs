use send_and_swap_to_vusd::send_and_swap_to_v_usd;
use shared::{error::Error, event::Event, soroban_data::AnySimpleSorobanData};
use soroban_sdk::{Address, BytesN, Env};

use crate::{
    events::Swapped,
    internal::method::{receive_and_swap_from_v_usd, send_and_swap_to_vusd},
    storage::bridge::Bridge,
};

pub fn swap(
    env: Env,
    sender: Address,
    amount: u128,
    token: BytesN<32>,
    receive_token: BytesN<32>,
    recipient: Address,
    receive_amount_min: u128,
) -> Result<(), Error> {
    Bridge::get(&env)?.assert_can_swap()?;
    sender.require_auth();

    let v_usd_amount = send_and_swap_to_v_usd(&env, &token, &sender, amount)?;
    let receive_amount = receive_and_swap_from_v_usd(
        &env,
        &receive_token,
        &recipient,
        v_usd_amount,
        receive_amount_min,
    )?;

    Swapped {
        sender,
        recipient,
        send_token: token,
        receive_token,
        send_amount: amount,
        receive_amount,
    }
    .publish(&env);

    Ok(())
}
