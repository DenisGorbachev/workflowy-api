//! This crate provides a basic [`Client`] for a [Workflowy API](https://beta.workflowy.com/api-reference).

#![deny(clippy::arithmetic_side_effects)]
#![deny(unused_crate_dependencies)]

mod client;

pub use client::*;

mod key;

pub use key::*;

mod limiter;

pub use limiter::*;

mod common;

pub use common::*;

mod request;

pub use request::*;

mod response;

pub use response::*;

#[cfg(test)]
mod integration_tests;
