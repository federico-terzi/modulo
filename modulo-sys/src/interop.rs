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

pub const FieldType_ROW: FieldType = 0;
pub const FieldType_LABEL: FieldType = 1;
pub const FieldType_TEXT: FieldType = 2;
pub const FieldType_CHOICE: FieldType = 3;
pub const FieldType_CHECKBOX: FieldType = 4;
pub type FieldType = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LabelMetadata {
    pub text: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TextMetadata {
    pub defaultText: *const ::std::os::raw::c_char,
    pub multiline: ::std::os::raw::c_int,
}

pub const ChoiceType_DROPDOWN: ChoiceType = 0;
pub const ChoiceType_LIST: ChoiceType = 1;
pub type ChoiceType = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ChoiceMetadata {
    pub values: *const *const ::std::os::raw::c_char,
    pub valueSize: ::std::os::raw::c_int,
    pub defaultValue: *const ::std::os::raw::c_char,
    pub choiceType: ChoiceType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FieldMetadata {
    pub id: *const ::std::os::raw::c_char,
    pub fieldType: FieldType,
    pub specific: *const ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RowMetadata {
    pub fields: *const FieldMetadata,
    pub fieldSize: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FormMetadata {
    pub windowTitle: *const ::std::os::raw::c_char,
    pub iconPath: *const ::std::os::raw::c_char,
    pub fields: *const FieldMetadata,
    pub fieldSize: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ValuePair {
    pub id: *const ::std::os::raw::c_char,
    pub value: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SearchItem {
    pub id: *const ::std::os::raw::c_char,
    pub label: *const ::std::os::raw::c_char,
    pub trigger: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SearchResults {
    pub items: *const SearchItem,
    pub itemSize: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SearchMetadata {
    pub windowTitle: *const ::std::os::raw::c_char,
    pub iconPath: *const ::std::os::raw::c_char,
}

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
