#![warn(missing_docs)]
extern crate num;
mod envelope;
mod adsr;
mod util;
pub use envelope::*;
pub use adsr::*;
pub use util::*;

//TODO: Docs