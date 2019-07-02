use chain_core::init::coin::Coin;
use chain_core::tx::data::address::ExtendedAddr;
use client_common::balance::BalanceChange;
use client_common::{ErrorKind, Result, Storage};
use parity_codec::{Decode, Encode};

const KEYSPACE: &str = "index_balance";

/// Exposes functionalities for managing balances
///
/// Stores `address -> balance` mapping
#[derive(Default, Clone)]
pub struct BalanceService<S: Storage> {
    storage: S,
}

impl<S> BalanceService<S>
where
    S: Storage,
{
    /// Creates a new instance of balance service
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    /// Retrieves current balance for given address
    pub fn get(&self, address: &ExtendedAddr) -> Result<Coin> {
        self.storage
            .get(KEYSPACE, address.encode())?
            .map(|bytes| {
                Ok(Coin::decode(&mut bytes.as_slice()).ok_or(ErrorKind::DeserializationError)?)
            })
            .unwrap_or_else(|| Ok(Coin::zero()))
    }

    /// Changes balance for an address with given balance change
    pub fn change(&self, address: &ExtendedAddr, change: &BalanceChange) -> Result<()> {
        self.storage
            .fetch_and_update(KEYSPACE, address.encode(), |value| {
                let current = value
                    .map(|mut bytes| -> Result<Coin> {
                        Ok(Coin::decode(&mut bytes).ok_or(ErrorKind::DeserializationError)?)
                    })
                    .unwrap_or_else(|| Ok(Coin::zero()))?;
                let new = (current + change)?;
                Ok(Some(new.encode()))
            })
            .map(|_| ())
    }

    /// Clears all storage
    pub fn clear(&self) -> Result<()> {
        self.storage.clear(KEYSPACE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chain_core::init::coin::Coin;
    use client_common::balance::BalanceChange;
    use client_common::storage::MemoryStorage;

    #[test]
    fn check_flow() {
        let balance_service = BalanceService::new(MemoryStorage::default());
        let address = ExtendedAddr::BasicRedeem(Default::default());

        assert_eq!(Coin::zero(), balance_service.get(&address).unwrap());

        assert!(balance_service
            .change(&address, &BalanceChange::Incoming(Coin::new(30).unwrap()))
            .is_ok());
        assert_eq!(
            Coin::new(30).unwrap(),
            balance_service.get(&address).unwrap()
        );

        assert!(balance_service
            .change(&address, &BalanceChange::Outgoing(Coin::new(20).unwrap()))
            .is_ok());
        assert_eq!(
            Coin::new(10).unwrap(),
            balance_service.get(&address).unwrap()
        );

        assert!(balance_service.clear().is_ok());
        assert_eq!(Coin::zero(), balance_service.get(&address).unwrap());
    }
}
