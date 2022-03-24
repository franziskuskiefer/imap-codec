#![deny(missing_debug_implementations)]

use codec::Encode;

#[cfg(feature = "arbitrary")]
pub(crate) mod arbitrary;
pub mod codec;
mod parse;
mod state;
pub(crate) mod types;
mod utils;

pub use types::{command::Command, response::Response};

/// This module exposes nom parsers for the formal syntax of IMAP,
/// see https://datatracker.ietf.org/doc/html/rfc3501#section-9
///
/// This module is only available when the feature "nomx" was specified.
/// In this case, nom types is also re-exported via imap-codec::nom.
#[cfg(feature = "nomx")]
pub mod rfc3501 {
    // As in https://datatracker.ietf.org/doc/html/rfc3501#section-9
    pub use crate::parse::address::{addr_adl, address};
    // ...
    pub use crate::parse::envelope::envelope;
    // ...
}

#[cfg(feature = "nomx")]
pub use nom;
