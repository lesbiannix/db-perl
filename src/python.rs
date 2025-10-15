//! Python bindings for the transport-api-wrapper.

use pyo3::prelude::*;
use tokio;
use crate::client::Client;

#[pyfunction]
fn locations(query: String) -> PyResult<String> {
    let client = Client::new();
    let locations = crate::runtime::RUNTIME
        .block_on(client.locations(&query))
        .unwrap();

    Ok(format!("{:#?}", locations))
}

#[pymodule]
fn transport_api_wrapper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(locations, m)?)?;
    Ok(())
}