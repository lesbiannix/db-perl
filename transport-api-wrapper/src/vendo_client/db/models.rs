//! This module contains the data models for the `Db` profile.
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "ResC")]
pub struct JourneyQueryResponse {
    #[serde(rename = "Connections")]
    pub connections: Connections,
}

#[derive(Deserialize, Debug)]
pub struct Connections {
    #[serde(rename = "Connection")]
    pub connection: Vec<Journey>,
}

#[derive(Deserialize, Debug)]
pub struct Journey {
    #[serde(rename = "Overview")]
    pub overview: Overview,
}

#[derive(Deserialize, Debug)]
pub struct Overview {
    #[serde(rename = "Leg")]
    pub legs: Vec<Leg>,
}

#[derive(Deserialize, Debug)]
pub struct Leg {
    pub origin: Stop,
    pub destination: Stop,
    pub line: Line,
    pub departure: String,
    pub arrival: String,
}

#[derive(Deserialize, Debug)]
pub struct Stop {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Line {
    pub name: String,
}