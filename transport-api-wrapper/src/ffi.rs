//! The C-compatible FFI layer for the transport-api-wrapper.

use std::ffi::{CStr, CString};
use tokio;
use std::os::raw::c_char;
use crate::client::Client;

#[no_mangle]
pub extern "C" fn locations(query: *const c_char) -> *mut c_char {
    let query = unsafe {
        assert!(!query.is_null());
        CStr::from_ptr(query)
    };

    let client = Client::new();
    let locations = crate::runtime::RUNTIME.block_on(client.locations(query.to_str().unwrap()));

    let c_string = CString::new(format!("{:#?}", locations)).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}