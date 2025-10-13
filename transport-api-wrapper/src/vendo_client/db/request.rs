//! This module handles the API requests for the `Db` profile.
use crate::error::Error;
use crate::vendo_client::db::models::JourneyQueryResponse;
use reqwest::Client;

const BASE_URL: &str = "https://reiseauskunft.bahn.de/bin/mgate.exe";

pub async fn send(method: &str, body: String) -> Result<JourneyQueryResponse, Error> {
    let client = Client::new();
    let res = client
        .post(BASE_URL)
        .header("Content-Type", "application/xml")
        .body(format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <ReqC ver="1.16" prod="HAFAS" lang="DE">
                <Client id="DB" type="IPH" v="3000000" />
                <ExtaGeoloc />
                <TripSearch>
                    {}
                </TripSearch>
            </ReqC>"#,
            body
        ))
        .send()
        .await?
        .text()
        .await?;

    let response: JourneyQueryResponse = quick_xml::de::from_str(&res)?;
    Ok(response)
}