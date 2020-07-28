#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::{c_void, c_int, c_char};

// Native bindings

#[allow(improper_ctypes)]
#[link(name = "modulosys", kind = "static")]
extern "C" {
    // FORM
    pub(crate) fn interop_show_form(
        metadata: *const FormMetadata,
        callback: extern "C" fn(
            values: *const ValuePair,
            size: c_int,
            map: *mut c_void,
        ),
        map: *mut c_void,
    );

    // SEARCH
    pub(crate) fn interop_show_search(
        metadata: *const SearchMetadata,
        callback: extern "C" fn(
            query: *const c_char,
            app: *const c_void,
            data: *const c_void,
        ),
        items: *const c_void,
    );

    pub(crate) fn update_items(
        app: *const c_void,
        items: *const SearchItem,
        itemCount: c_int,
    );
}

