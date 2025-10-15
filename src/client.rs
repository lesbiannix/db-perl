use crate::error::Error;
use crate::models::{Departure, Stop, DeparturesResponse};
use reqwest::Client as ReqwestClient;

const BASE_URL: &str = "https://v6.db.transport.rest";

/// The main client for interacting with the transport.rest API.
///
/// This client provides methods for accessing the various endpoints of the API,
/// such as finding locations and getting departures.
#[derive(Default)]
pub struct Client {
    client: ReqwestClient,
}

impl Client {
    /// Creates a new `Client` with a default `reqwest::Client`.
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
        }
    }

    /// Finds locations based on a query.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query (e.g., a station name).
    pub async fn locations(&self, query: &str) -> Result<Vec<Stop>, Error> {
        let url = format!("{}/locations", BASE_URL);
        let response = self
            .client
            .get(&url)
            .query(&[("query", query)])
            .send()
            .await?
            .json::<Vec<Stop>>()
            .await?;
        Ok(response)
    }

    /// Gets departures for a specific stop.
    ///
    /// # Arguments
    ///
    /// * `stop_id` - The ID of the stop.
    pub async fn stop_departures(&self, stop_id: &str) -> Result<Vec<Departure>, Error> {
        let url = format!("{}/stops/{}/departures", BASE_URL, stop_id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<DeparturesResponse>()
            .await?;
        Ok(response.departures)
    }
}