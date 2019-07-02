use client_common::{Result, Storage};
use parity_codec::{Decode, Encode};

const KEYSPACE: &str = "index_global_state";
const LAST_BLOCK_HEIGHT: &str = "last_block_height";

/// Exposes functionalities for managing client's global state
#[derive(Default, Clone)]
pub struct GlobalStateService<S: Storage> {
    storage: S,
}

impl<S> GlobalStateService<S>
where
    S: Storage,
{
    /// Creates new instance of global state service
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    /// Returns currently stored last block height
    pub fn last_block_height(&self) -> Result<Option<u64>> {
        let last_block_height = self
            .storage
            .get(KEYSPACE, LAST_BLOCK_HEIGHT)?
            .and_then(|bytes| u64::decode(&mut bytes.as_slice()));

        Ok(last_block_height)
    }

    /// Updates last block height with given value and returns old value
    pub fn set_last_block_height(&self, last_block_height: u64) -> Result<Option<u64>> {
        let bytes = last_block_height.encode();

        let old_last_block_height = self
            .storage
            .set(KEYSPACE, LAST_BLOCK_HEIGHT, bytes)?
            .and_then(|bytes| u64::decode(&mut bytes.as_slice()));

        Ok(old_last_block_height)
    }

    /// Clears all storage
    pub fn clear(&self) -> Result<()> {
        self.storage.clear(KEYSPACE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use client_common::storage::MemoryStorage;

    #[test]
    fn check_flow() {
        let global_state_service = GlobalStateService::new(MemoryStorage::default());

        assert_eq!(None, global_state_service.last_block_height().unwrap());
        assert_eq!(None, global_state_service.set_last_block_height(5).unwrap());
        assert_eq!(
            5,
            global_state_service.last_block_height().unwrap().unwrap()
        );
        assert!(global_state_service.clear().is_ok());
        assert_eq!(None, global_state_service.last_block_height().unwrap());
    }
}
