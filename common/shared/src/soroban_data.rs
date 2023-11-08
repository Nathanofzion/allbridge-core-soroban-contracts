use soroban_env_common::StorageType;
use soroban_sdk::{Env, IntoVal, Symbol, TryFromVal, Val};

use crate::error::Error;

pub trait SorobanData {}
pub trait SorobanSimpleData {}

// ----------------------------------------- //

pub trait SymbolKey {
    const STORAGE_KEY: Symbol;
}

pub trait DataStorageType {
    const STORAGE_TYPE: StorageType;
}

pub trait BumpInfo {
    /// @see https://github.com/stellar/soroban-examples/blob/main/token/src/storage_types.rs#L8
    /// @see https://github.com/stellar/soroban-examples/blob/7a7cc6268ada55113ce0b82a3ae4405f7ec8b8f0/token/src/balance.rs#L2
    const BUMP_AMOUNT: u32;
    const LIFETIME_THRESHOLD: u32;
}

// ----------------------------------------- //

pub trait AnySimpleSorobanData: TryFromVal<Env, Val> + IntoVal<Env, Val> + Sized {
    fn get(env: &Env) -> Result<Self, Error>;
    fn save(&self, env: &Env);
    fn bump(env: &Env);

    fn has(env: &Env) -> bool;
    fn update<F>(env: &Env, handler: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Error>;
}

pub trait AnySorobanData:
    TryFromVal<Env, Val> + IntoVal<Env, Val> + DataStorageType + Sized
{
    fn get_by_key<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> Result<Self, Error>;
    fn save_by_key<K: IntoVal<Env, Val>>(&self, env: &Env, key: &K);
    fn bump_by_key<K: IntoVal<Env, Val>>(env: &Env, key: &K);

    #[inline]
    fn has_by_key<K: IntoVal<Env, Val>>(env: &Env, key: K) -> bool {
        Self::get_by_key(env, &key).is_ok()
    }

    fn update_by_key<F, K>(env: &Env, key: &K, handler: F) -> Result<(), Error>
    where
        K: IntoVal<Env, Val>,
        F: FnOnce(&mut Self) -> Result<(), Error>,
    {
        {
            let mut object = Self::get_by_key(env, key)?;

            handler(&mut object)?;

            object.save_by_key(env, key);

            Ok(())
        }
    }
}

// ----------------------------------------- //

impl<T> AnySorobanData for T
where
    T: TryFromVal<Env, Val> + IntoVal<Env, Val> + DataStorageType + BumpInfo + SorobanData + Sized,
{
    fn get_by_key<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> Result<Self, Error> {
        let result = (match Self::STORAGE_TYPE {
            StorageType::Instance => env.storage().instance().get(key),
            StorageType::Temporary => env.storage().temporary().get(key),
            StorageType::Persistent => env.storage().persistent().get(key),
        })
        .ok_or(Error::Uninitialized)?;

        Self::bump_by_key(env, key);

        Ok(result)
    }

    fn save_by_key<K: IntoVal<Env, Val>>(&self, env: &Env, key: &K) {
        match Self::STORAGE_TYPE {
            StorageType::Instance => env.storage().instance().set(key, self),
            StorageType::Temporary => env.storage().temporary().set(key, self),
            StorageType::Persistent => env.storage().persistent().set(key, self),
        };

        Self::bump_by_key(env, key);
    }

    fn bump_by_key<K: IntoVal<Env, Val>>(env: &Env, key: &K) {
        match Self::STORAGE_TYPE {
            StorageType::Instance => env
                .storage()
                .instance()
                .bump(Self::LIFETIME_THRESHOLD, Self::BUMP_AMOUNT),
            StorageType::Temporary => {
                env.storage()
                    .temporary()
                    .bump(key, Self::LIFETIME_THRESHOLD, Self::BUMP_AMOUNT)
            }
            StorageType::Persistent => {
                env.storage()
                    .persistent()
                    .bump(key, Self::LIFETIME_THRESHOLD, Self::BUMP_AMOUNT)
            }
        }
    }
}

impl<T> AnySimpleSorobanData for T
where
    T: SymbolKey + AnySorobanData + SorobanSimpleData,
{
    #[inline(always)]
    fn get(env: &Env) -> Result<Self, Error> {
        Self::get_by_key(env, &Self::STORAGE_KEY)
    }

    #[inline(always)]
    fn has(env: &Env) -> bool {
        Self::has_by_key(env, Self::STORAGE_KEY)
    }

    #[inline(always)]
    fn save(&self, env: &Env) {
        self.save_by_key(env, &Self::STORAGE_KEY);
    }

    #[inline(always)]
    fn update<F>(env: &Env, handler: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Error>,
    {
        Self::update_by_key(env, &Self::STORAGE_KEY, handler)
    }

    #[inline(always)]
    fn bump(env: &Env) {
        Self::bump_by_key(env, &Self::STORAGE_KEY);
    }
}
