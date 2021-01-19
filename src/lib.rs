// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A thicc message bus for Rust.

#![deny(missing_docs)]
//#![deny(missing_doc_example)] <-- for later, when I'm swole

use std::collections::HashMap;

type Handler = fn(Vec<u8>) -> Vec<u8>;
type Res = Vec<u8>; //TODO should be the reciever end of a channel

/// Represents a message that couldn't be sent since the addressee doesn't exist.
#[derive(PartialEq, Debug)]
pub struct DeadLetter(String);

/// The main struct used by Chonky, create a new one with Chonky::new().
pub struct Chonky {
    addressees: HashMap<String, Handler>,
}

impl Chonky {
    /// Creates a new instance of Chonky.
    /// Most likely you'll only need one of these per application and either share or make static.
    pub fn new() -> Chonky {
        Chonky {
            addressees: HashMap::new(),
        }
    }

    /// Adds a new addrressee to this instance.
    /// Note: This function panics if there is already an addressee registered for this address.
    pub fn register_addressee(&mut self, address: String, handler: Handler) {
        self.addressees.insert(address, handler);
    }

    /// Posts a message to the given address.
    pub fn post(&self, address: String, message: Vec<u8>) -> Result<Res, DeadLetter> {
        let res = self.addressees.get(&address);
        match res {
            Some(handler) => Ok(handler(message)),
            None => Err(DeadLetter(format!("Could not find {}", address))),
        }
    }
}
