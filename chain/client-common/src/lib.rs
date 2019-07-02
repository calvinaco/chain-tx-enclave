#![deny(missing_docs, unsafe_code, unstable_features)]
//! This crate contains all the common types and utilities used by other `client-*` crates.
mod transaction;

pub mod balance;
pub mod error;
pub mod storage;
pub mod tendermint;

#[doc(inline)]
pub use error::{Error, ErrorKind, Result};
#[doc(inline)]
pub use storage::{SecureStorage, Storage};
pub use transaction::Transaction;
