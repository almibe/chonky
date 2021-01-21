// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A thicc message bus for Rust.

#![deny(missing_docs)]
//#![deny(missing_doc_example)] <-- for later, when I'm swole

use std::collections::HashMap;
use std::any::Any;

/// The request/response type for Chonky.
pub type Messages = Box<dyn Iterator<Item = Box<dyn Any>>>;
/// The type of a handler for Chonky.
pub type Handler = fn(Messages) -> Result<Messages, HandlerError>;
/// Represents an error from a Handler function.
#[derive(PartialEq, Debug)]
pub struct HandlerError(String);
/// Represents a message that couldn't be sent since the addressee doesn't exist.
#[derive(PartialEq, Debug)]
pub struct DeadLetter(String);

/// Represents an error that can be returned from a call to the post method.
#[derive(PartialEq, Debug)]
pub enum ChonkyError {
    /// Wrapper for HandlerErrors.
    HandlerError(HandlerError),
    /// Wrapper for DeadLetter errors.
    DeadLetter(DeadLetter),
}

/// The main struct used by Chonky, create a new one with Chonky::new().
pub struct Chonky {
    addressees: HashMap<String, Handler>,
}

impl Default for Chonky {
    fn default() -> Self {
        Self::new()
    }
}

/// A helper fn that converts a given Iterator of a generic type to Messages.
pub fn to_messages<T: 'static>(input: impl Iterator<Item = T> + 'static) -> Messages {
    let itr = input.map(|i| Box::new(i) as Box<dyn Any>);
    Box::new(itr)
}

/// A helper fn that converts a given stream of Messages into an Iterator of a generic type.
/// TODO should return a Result
pub fn from_messages<T: 'static + Clone>(messages: Messages) -> impl Iterator<Item = T> {
    messages.map(|m| match m.downcast_ref::<T>() {
        Some(t) => {
            t.clone()
        }
        None => {
            todo!()
        }
    })
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
        if self.addressees.contains_key(&address) {
            panic!("Cannot have two addressees with address {}", address);
        } else {
            self.addressees.insert(address, handler);
        }
    }

    /// Posts a message to the given address.
    pub fn post(&self, address: String, message: Messages) -> Result<Messages, ChonkyError> {
        let addressee = self.addressees.get(&address);
        match addressee {
            Some(handler) => {
                let result = handler(message);
                match result {
                    Ok(messages) => Ok(messages),
                    Err(error) => Err(ChonkyError::HandlerError(error)),
                }
            }
            None => Err(ChonkyError::DeadLetter(DeadLetter(format!(
                "Could not find {}",
                address
            )))),
        }
    }
}
