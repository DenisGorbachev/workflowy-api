//! This is a module-level comment for a Rust lib

#![deny(clippy::arithmetic_side_effects)]
#![deny(unused_crate_dependencies)]

mod client;

pub use client::*;

mod common;

pub use common::*;

mod request;

pub use request::*;

mod response;

pub use response::*;

#[cfg(test)]
mod integration_tests;
