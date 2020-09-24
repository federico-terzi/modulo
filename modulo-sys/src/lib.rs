pub mod form;
pub(crate) mod interop;
pub mod search;

use std::ffi::c_void;

pub(crate) trait Interoperable {
    fn as_ptr(&self) -> *const c_void;
}
