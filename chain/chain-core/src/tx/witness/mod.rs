/// Witness for Merklized Abstract Syntax Trees (MAST) + Schnorr
pub mod tree;

use parity_scale_codec::{Decode, Encode, Error, Input, Output};
use std::fmt;
use std::prelude::v1::Vec;
// TODO: switch to normal signatures + explicit public key
use secp256k1::{self, recovery::RecoverableSignature, schnorrsig::SchnorrSignature};

use crate::common::Proof;
use crate::tx::witness::tree::{RawPubkey, RawSignature};

pub type EcdsaSignature = RecoverableSignature;

/// A transaction witness is a vector of input witnesses
#[derive(Debug, Default, PartialEq, Eq, Clone, Encode, Decode)]
pub struct TxWitness(Vec<TxInWitness>);

impl TxWitness {
    /// creates an empty witness (for testing/tools)
    pub fn new() -> Self {
        TxWitness::default()
    }
}
impl From<Vec<TxInWitness>> for TxWitness {
    fn from(v: Vec<TxInWitness>) -> Self {
        TxWitness(v)
    }
}
impl ::std::iter::FromIterator<TxInWitness> for TxWitness {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = TxInWitness>,
    {
        TxWitness(Vec::from_iter(iter))
    }
}
impl ::std::ops::Deref for TxWitness {
    type Target = Vec<TxInWitness>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for TxWitness {
    fn deref_mut(&mut self) -> &mut Vec<TxInWitness> {
        &mut self.0
    }
}

// normally should be some structure: e.g. indicate a type of signature
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TxInWitness {
    TreeSig(SchnorrSignature, Proof<RawPubkey>),
}

impl fmt::Display for TxInWitness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Encode for TxInWitness {
    fn encode_to<W: Output>(&self, dest: &mut W) {
        match *self {
            TxInWitness::TreeSig(ref schnorrsig, ref proof) => {
                dest.push_byte(0);
                schnorrsig.serialize_default().encode_to(dest);
                proof.encode_to(dest);
            }
        }
    }
}

impl Decode for TxInWitness {
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        let tag = input.read_byte()?;
        match tag {
            0 => {
                let raw_sig = RawSignature::decode(input)?;
                let schnorrsig = SchnorrSignature::from_default(&raw_sig)
                    .map_err(|_| Error::from("Unable to parse schnorr signature"))?;
                let proof = Proof::decode(input)?;
                Ok(TxInWitness::TreeSig(schnorrsig, proof))
            }
            _ => Err(Error::from("Invalid tag")),
        }
    }
}
