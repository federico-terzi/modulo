/*
 * This file is part of modulo.
 *
 * Copyright (C) 2020-2021 Federico Terzi
 *
 * modulo is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * modulo is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with modulo.  If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::{c_char, c_int, c_void};

// Native bindings

#[allow(improper_ctypes)]
#[link(name = "modulosys", kind = "static")]
extern "C" {
    // FORM
    pub(crate) fn interop_show_form(
        metadata: *const FormMetadata,
        callback: extern "C" fn(values: *const ValuePair, size: c_int, map: *mut c_void),
        map: *mut c_void,
    );

    // SEARCH
    pub(crate) fn interop_show_search(
        metadata: *const SearchMetadata,
        search_callback: extern "C" fn(
            query: *const c_char,
            app: *const c_void,
            data: *const c_void,
        ),
        items: *const c_void,
        result_callback: extern "C" fn(id: *const c_char, result: *mut c_void),
        result: *mut c_void,
    );

    pub(crate) fn update_items(app: *const c_void, items: *const SearchItem, itemCount: c_int);
}
