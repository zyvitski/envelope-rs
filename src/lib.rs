#![warn(missing_docs)]

///!A crate for representing type generic Envelope generators.

extern crate num;
mod envelope;
mod adsr;
mod util;
pub use envelope::*;
pub use adsr::*;
pub use util::*;
