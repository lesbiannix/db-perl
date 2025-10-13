//! This module contains the implementation for the `Db` profile.
use crate::error::Error;
use crate::vendo_client::db::models::Journey;

pub mod request;
pub mod models;

pub async fn journeys(from: &str, to: &str) -> Result<Vec<Journey>, Error> {
    let res = request::send(
        "JourneyQuery",
        format!(
            r#"<JourneyQueryRequest>
                <From>
                    <Station uic_code="{}" />
                </From>
                <To>
                    <Station uic_code="{}" />
                </To>
                <Products>
                    <Product>ICE</Product>
                    <Product>IC</Product>
                    <Product>EC</Product>
                    <Product>RE</Product>
                    <Product>RB</Product>
                    <Product>S</Product>
                </Products>
            </JourneyQueryRequest>"#,
            from, to
        ),
    )
    .await?;
    Ok(res.connections.connection)
}