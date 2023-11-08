use bridge_storage::*;
use shared::{error::Error, soroban_data::AnySimpleSorobanData};
use soroban_sdk::{BytesN, Env};

use crate::storage::config::Config;

pub fn set_other_chain_ids(env: Env, other_chain_ids: BytesN<32>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Config::update(&env, |config| {
        config.other_chain_ids = other_chain_ids;
        Ok(())
    })
}
