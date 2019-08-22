use crate::tendermint::types::QueryResult;
use crate::tendermint::types::*;
use crate::Result;

/// Makes remote calls to tendermint (backend agnostic)
pub trait Client: Send + Sync {
    /// Makes `genesis` call to tendermint
    fn genesis(&self) -> Result<Genesis>;

    /// Makes `status` call to tendermint
    fn status(&self) -> Result<Status>;

    /// Makes `block` call to tendermint
    fn block(&self, height: u64) -> Result<Block>;

    /// Makes batched `block` call to tendermint
    fn block_batch<T: Iterator<Item = u64>>(&self, heights: T) -> Result<Vec<Block>>;

    /// Makes `block_results` call to tendermint
    fn block_results(&self, height: u64) -> Result<BlockResults>;

    /// Makes batched `block_results` call to tendermint
    fn block_results_batch<T: Iterator<Item = u64>>(&self, heights: T)
        -> Result<Vec<BlockResults>>;

    /// Makes `broadcast_tx_sync` call to tendermint
    fn broadcast_transaction(&self, transaction: &[u8]) -> Result<()>;

    /// Makes `abci_query` call to tendermint
    fn query(&self, path: &str, data: &[u8]) -> Result<QueryResult>;
}
