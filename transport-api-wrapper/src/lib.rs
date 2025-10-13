//! A Rust wrapper for the transport.rest API.
//!
//! This crate provides a simple and easy-to-use interface for interacting with the
//! transport.rest API, which offers access to public transport data from various
//! providers.

pub mod client;
pub mod error;
pub mod models;
pub mod via_decoder;
pub mod vendo_client;
pub mod tickets_client;