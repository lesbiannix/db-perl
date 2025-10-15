//! Node.js bindings for the transport-api-wrapper.

use neon::prelude::*;
use crate::client::Client;

fn locations(mut cx: FunctionContext) -> JsResult<JsString> {
    let query = cx.argument::<JsString>(0)?.value();
    let client = Client::new();
    let locations = crate::runtime::RUNTIME
        .block_on(client.locations(&query))
        .unwrap();

    Ok(cx.string(format!("{:#?}", locations)))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("locations", locations)?;
    Ok(())
}