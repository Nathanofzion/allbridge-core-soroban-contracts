use soroban_sdk::{Address, Bytes, BytesN, Env};

mod bump;
mod hash_message;
mod hash_with_sender;
pub mod num;
pub mod require;

pub use bump::*;
pub use hash_message::*;
pub use hash_with_sender::*;

pub fn is_bytesn32_empty(bytesn: &BytesN<32>) -> bool {
    bytesn.is_empty() || bytesn.to_array() == [0; 32]
}

pub fn bytes_to_slice<const N: usize>(bytes: Bytes) -> [u8; N] {
    let mut xdr_slice: [u8; N] = [0; N];
    bytes.copy_into_slice(&mut xdr_slice);

    xdr_slice
}

pub fn bytesn_to_slice<const N: usize>(bytes: BytesN<N>) -> [u8; N] {
    let mut xdr_slice: [u8; N] = [0; N];
    bytes.copy_into_slice(&mut xdr_slice);

    xdr_slice
}

pub fn merge_slices_by_half<const N: usize, const R: usize>(a: &[u8; N], b: &[u8; N]) -> [u8; R] {
    let mut slice = [0u8; R];

    slice[..N].copy_from_slice(a);
    slice[N..].copy_from_slice(b);

    slice
}

pub fn get_token_client_by_token_bytesn<'a>(
    env: &'a Env,
    token: &BytesN<32>,
) -> soroban_sdk::token::Client<'a> {
    let token_address = Address::from_contract_id(token);

    soroban_sdk::token::Client::new(env, &token_address)
}