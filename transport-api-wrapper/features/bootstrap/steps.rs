use cucumber::{given, when, then, World};
use std::collections::HashMap;
use tokio;
use transport_api_wrapper::client::Client;
use transport_api_wrapper::models::Stop;

#[derive(Debug, Default, World)]
pub struct LocationsWorld {
    query: String,
    locations: Vec<Stop>,
}

#[given(expr = "a location query {string}")]
fn a_location_query(w: &mut LocationsWorld, query: String) {
    w.query = query;
}

#[when("I search for locations")]
async fn search_for_locations(w: &mut LocationsWorld) {
    let client = Client::new();
    w.locations = client.locations(&w.query).await.unwrap();
}

#[then("I should receive a list of locations")]
fn should_receive_locations(w: &mut LocationsWorld) {
    assert!(!w.locations.is_empty());
}

#[then(expr = "the list should contain {string}")]
fn should_contain_location(w: &mut LocationsWorld, name: String) {
    assert!(w.locations.iter().any(|l| l.name == name));
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(LocationsWorld::run("features/locations.feature"));
}