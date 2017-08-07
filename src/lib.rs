//! Squirrel - the cross-platform installation and update library

//#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

extern crate semver;
extern crate sha2;

pub use release_entry::{ReleaseEntry};

mod release_entry;