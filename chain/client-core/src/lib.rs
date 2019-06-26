#![deny(missing_docs, unsafe_code, unstable_features)]
//! # Crypto.com Chain Client
//!
//! This crate provides `WalletClient` trait which is responsible for interacting with transaction index and provide
//! following functionalities on per-wallet basis:
//!
//! - Wallet creation
//! - Address generation
//! - Balance tracking
//! - Transaction history
//! - Transaction creation and signing (with automatic unspent transaction selection)
pub mod input_selection;
pub mod key;
pub mod service;
pub mod signer;
pub mod transaction_builder;
pub mod unspent_transactions;
pub mod wallet;

#[doc(inline)]
pub use input_selection::InputSelectionStrategy;
#[doc(inline)]
pub use key::{PrivateKey, PublicKey};
#[doc(inline)]
pub use signer::Signer;
#[doc(inline)]
pub use transaction_builder::TransactionBuilder;
#[doc(inline)]
pub use unspent_transactions::{SelectedUnspentTransactions, UnspentTransactions};
#[doc(inline)]
pub use wallet::{MultiSigWalletClient, WalletClient};

use secp256k1::{All, Secp256k1};

thread_local! { pub(crate) static SECP: Secp256k1<All> = Secp256k1::new(); }
