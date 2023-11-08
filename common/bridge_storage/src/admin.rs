use proc_macros::{
    bump_info_instance, data_storage_type, symbol_key, SorobanData, SorobanSimpleData,
};
use shared::{error::Error, soroban_data::SimpleSorobanData};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(SorobanData, SorobanSimpleData)]
#[symbol_key("Admin")]
#[data_storage_type(Instance)]
#[bump_info_instance]
pub struct Admin(pub Address);

impl AsRef<Address> for Admin {
    fn as_ref(&self) -> &Address {
        &self.0
    }
}

impl Admin {
    #[inline]
    pub fn require_exist_auth(env: &Env) -> Result<(), Error> {
        let admin = Self::get(env)?;
        admin.0.require_auth();
        Ok(())
    }

    pub fn require_auth(&self) {
        self.0.require_auth();
    }

    #[inline]
    pub fn as_address(&self) -> Address {
        self.0.clone()
    }
}