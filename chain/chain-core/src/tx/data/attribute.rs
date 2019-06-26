use parity_codec::{Decode, Encode, Input, Output};
use serde::{Deserialize, Serialize};

use crate::tx::data::access::TxAccessPolicy;

/// Tx extra metadata, e.g. network ID
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TxAttributes {
    pub chain_hex_id: u8,
    pub allowed_view: Vec<TxAccessPolicy>,
    // TODO: other attributes, e.g. versioning info
}

impl Encode for TxAttributes {
    fn encode_to<W: Output>(&self, dest: &mut W) {
        dest.push_byte(0);
        dest.push_byte(2);
        dest.push_byte(self.chain_hex_id);
        self.allowed_view.encode_to(dest);
    }
}

impl Decode for TxAttributes {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        let tag = input.read_byte()?;
        let constructor_len = input.read_byte()?;
        match (tag, constructor_len) {
            (0, 2) => {
                let chain_hex_id: u8 = input.read_byte()?;
                let allowed_view: Vec<TxAccessPolicy> = Vec::decode(input)?;
                Some(TxAttributes::new_with_access(chain_hex_id, allowed_view))
            }
            _ => None,
        }
    }
}

impl TxAttributes {
    /// creates tx attributes
    pub fn new(chain_hex_id: u8) -> Self {
        TxAttributes {
            chain_hex_id,
            allowed_view: Vec::new(),
        }
    }

    /// creates tx attributes with access policy
    pub fn new_with_access(chain_hex_id: u8, allowed_view: Vec<TxAccessPolicy>) -> Self {
        TxAttributes {
            chain_hex_id,
            allowed_view,
        }
    }
}
