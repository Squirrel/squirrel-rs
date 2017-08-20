//! Squirrel - the cross-platform installation and update library

//#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate lazy_static;

/*
#[macro_use]
extern crate log;
*/

extern crate regex;
extern crate semver;
extern crate sha2;
extern crate url;

pub use release_entry::{ReleaseEntry};

mod hex;
mod release_entry;