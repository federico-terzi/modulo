pub mod form;
pub mod search;
pub(crate) mod interop;

use std::ffi::c_void;

pub(crate) trait Interoperable {
    fn as_ptr(&self) -> *const c_void;
}