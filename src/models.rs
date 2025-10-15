//! Contains the data models for the API responses.
use serde::Deserialize;

/// Represents a geographical location.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub r#type: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
}

/// Represents a stop or station.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub r#type: String,
    pub id: String,
    pub name: String,
    pub location: Location,
    pub products: Products,
}

/// Represents the products available at a stop.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Products {
    pub national_express: bool,
    pub national: bool,
    pub regional_express: bool,
    pub regional: bool,
    pub suburban: bool,
    pub bus: bool,
    pub ferry: bool,
    pub subway: bool,
    pub tram: bool,
    pub taxi: bool,
}

/// Represents the response for a departures request.
#[derive(Deserialize, Debug)]
pub struct DeparturesResponse {
    pub departures: Vec<Departure>,
}

/// Represents a single departure.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    pub trip_id: String,
    pub direction: Option<String>,
    pub line: Line,
    pub when: Option<String>,
    pub planned_when: String,
    pub delay: Option<i64>,
    pub platform: Option<String>,
    pub planned_platform: Option<String>,
    pub stop: Stop,
    pub remarks: Vec<Remark>,
}

/// Represents a line of transport.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub r#type: String,
    pub id: String,
    pub name: String,
    pub mode: String,
    pub product: String,
}

/// Represents a remark or note about a departure.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Remark {
    pub r#type: String,
    pub summary: Option<String>,
    pub text: String,
}