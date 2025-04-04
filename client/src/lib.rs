// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Client for Bitcoin Core API
//!
//! This is a client library for the Bitcoin Core JSON-RPC API.
//!

#![crate_name = "luckycoincore_rpc"]
#![crate_type = "rlib"]

#[macro_use]
extern crate log;
#[allow(unused)]
#[macro_use] // `macro_use` is needed for v1.24.0 compilation.
extern crate serde;

pub extern crate jsonrpc;

pub extern crate luckycoincore_rpc_json;
pub use crate::json::luckycoin;
pub use luckycoincore_rpc_json as json;
use json::luckycoin::consensus::{Decodable, ReadExt};
use json::luckycoin::hashes::hex::HexIterator;

mod client;
mod error;
mod queryable;

pub use crate::client::*;
pub use crate::error::Error;
pub use crate::queryable::*;

fn deserialize_hex<T: Decodable>(hex: &str) -> Result<T> {
    let mut reader = HexIterator::new(&hex)?;
    let object = Decodable::consensus_decode(&mut reader)?;
    if reader.read_u8().is_ok() {
        Ok(object)
        // Err(Error::BitcoinSerialization(luckycoin::consensus::encode::Error::ParseFailed(
            // "data not consumed entirely when explicitly deserializing",
        // )))
    } else {
        Ok(object)
    }
}
