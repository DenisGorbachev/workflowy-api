//! This is a module-level comment for a Rust lib

#![deny(clippy::arithmetic_side_effects)]
#![deny(unused_crate_dependencies)]

#[cfg(test)]
use tokio as _;

mod client;
pub use client::*;

pub mod common;
pub use common::*;

pub mod request;
pub use request::*;

pub mod response;
pub use response::*;

#[cfg(test)]
mod integration_tests;
