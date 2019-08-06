//! This crate contains messages exchanged in REQ-REP socket between chain-abci app to enclave wrapper server
use chain_core::state::account::DepositBondTx;
use chain_core::state::account::StakedState;
use chain_core::state::account::StakedStateOpWitness;
use chain_core::state::account::WithdrawUnbondedTx;
use chain_core::tx::data::Tx;
use chain_core::tx::data::TxId;
use chain_core::tx::witness::TxWitness;
use chain_core::tx::{fee::Fee, TxAux};
use chain_core::ChainInfo;
use chain_tx_validation::TxWithOutputs;

use parity_scale_codec::{Decode, Encode};

/// requests sent from chain-abci app to enclave wrapper server
/// FIXME: the variant will be smaller once the TX storage is on the enclave side
#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode)]
pub enum EnclaveRequest {
    /// a sanity check (sends the chain network ID -- last byte / two hex digits convention)
    /// during InitChain or startup (to test one connected to the correct process)
    /// FIXME: test genesis hash etc.
    CheckChain { chain_hex_id: u8 },
    /// "stateless" transaction validation requests (sends transaction + all required information)
    /// double-spent / BitVec check done in chain-abci
    /// FIXME: when sealing is done, sealed TX would probably be stored by enclave server, hence this should send TxPointers instead
    /// FIXME: only certain Tx types should be sent -> create a datatype / enum for it (probably after encrypted Tx data types)
    VerifyTx {
        tx: TxAux,
        account: Option<StakedState>,
        info: ChainInfo,
    },
}

/// reponses sent from enclave wrapper server to chain-abci app
/// TODO: better error responses?
#[derive(Encode, Decode)]
pub enum EnclaveResponse {
    /// returns OK if chain_hex_id matches the one embedded in enclave
    CheckChain(Result<(), ()>),
    /// returns the affected (account) state (if any) and paid fee if the TX is valid
    VerifyTx(Result<(Fee, Option<StakedState>), ()>),
    /// response if unsupported tx type is sent (e.g. unbondtx) -- TODO: probably unnecessary if there is a data type with a subset of TxAux
    UnsupportedTxType,
    /// response if the enclave failed to parse the request
    UnknownRequest,
}

/// ZMQ flags to be used in the socket connection
pub const FLAGS: i32 = 0;

/// TODO: rethink / should be direct communication with the enclave (rather than via abci+zmq)
#[derive(Encode, Decode)]
pub enum EncryptionRequest {
    TransferTx(Tx, TxWitness),
    DepositStake(DepositBondTx, TxWitness),
    WithdrawStake(WithdrawUnbondedTx, StakedState, StakedStateOpWitness),
}

/// TODO: rethink / should be direct communication with the enclave (rather than via abci+zmq)
#[derive(Encode, Decode)]
pub struct EncryptionResponse {
    pub tx: TxAux,
}

/// TODO: rethink / should be direct communication with the enclave (rather than via abci+zmq)
/// TODO: limit txs size + no of view keys in each TX?
#[derive(Encode, Decode)]
pub struct DecryptionRequestBody {
    pub txs: Vec<TxId>,
}

/// TODO: rethink / should be direct communication with the enclave (rather than via abci+zmq)
#[derive(Encode, Decode)]
pub struct DecryptionRequest {
    pub body: DecryptionRequestBody,
    /// ecdsa on the body in compact form?
    pub view_key_sig: [u8; 64],
}

/// TODO: rethink / should be direct communication with the enclave (rather than via abci+zmq)
#[derive(Encode, Decode)]
pub struct DecryptionResponse {
    pub txs: Vec<TxWithOutputs>,
}
