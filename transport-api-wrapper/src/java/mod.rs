//! Java bindings for the transport-api-wrapper.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use crate::client::Client;
use jni::into_raw::IntoRaw;
use tokio;

#[no_mangle]
pub extern "system" fn Java_com_example_transportapiwrapper_TransportApiWrapper_locations<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    query: JString<'local>,
) -> jstring {
    let query: String = env.get_string(query).unwrap().into();
    let client = Client::new();
    let locations = crate::runtime::RUNTIME
        .block_on(client.locations(&query))
        .unwrap();

    let output = env
        .new_string(format!("{:#?}", locations))
        .unwrap();
    output.into_raw()
}