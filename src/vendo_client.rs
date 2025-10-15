//! A Rust implementation of the `db-vendo-client`.
use crate::error::Error;
use reqwest::Client as ReqwestClient;

/// Represents the different API profiles that can be used.
pub enum Profile {
    Db,
    DbNav,
    DbWeb,
    DbBahnhof,
    DbRis,
}

pub mod db;

/// The client for interacting with the DB Vendo API.
pub struct Client {
    profile: Profile,
    client: ReqwestClient,
}

use crate::vendo_client::db::models::Journey;

impl Client {
    /// Creates a new `Client` with the specified profile.
    pub fn new(profile: Profile) -> Self {
        Self {
            profile,
            client: ReqwestClient::new(),
        }
    }

    /// Finds journeys from a `from` location to a `to` location.
    pub async fn journeys(&self, from: &str, to: &str) -> Result<Vec<Journey>, Error> {
        match self.profile {
            Profile::Db => db::journeys(from, to).await,
            _ => unimplemented!(),
        }
    }
}