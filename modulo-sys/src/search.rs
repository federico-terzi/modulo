use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_char, c_void};

pub mod types {
    #[derive(Debug)]
    pub struct SearchItem {
        pub id: String,
        pub label: String,
        //TODO pub search_text: String,
    }

    #[derive(Debug)]
    pub struct Search {
        pub title: String,
        pub items: Vec<SearchItem>,
    }
}

#[allow(dead_code)]
mod interop {
    use crate::interop::*;
    use super::types;
    use crate::Interoperable;
    use std::ffi::{c_void, CString};
    use std::os::raw::{c_char, c_int};
    use std::ptr::null;

    pub(crate) struct OwnedSearch {
        title: CString,

        _interop: Box<SearchMetadata>,
    }

    impl Interoperable for OwnedSearch {
        fn as_ptr(&self) -> *const c_void {
            &(*self._interop) as *const SearchMetadata as *const c_void
        }
    }

    impl From<types::Search> for OwnedSearch {
        fn from(search: types::Search) -> Self {
            let title = CString::new(search.title).expect("unable to convert search title to CString");

            let _interop = Box::new(SearchMetadata {
                windowTitle: title.as_ptr(),
            });

            Self {
                title,
                _interop,
            }
        }
    }
}

pub fn show(search: types::Search) -> Option<String> {
    use crate::interop::SearchMetadata;
    use crate::Interoperable;

    let owned_search: interop::OwnedSearch = search.into();
    let metadata: *const SearchMetadata = owned_search.as_ptr() as *const SearchMetadata;

    // TODO: show search
    // Callback parameter is called when the search text changes and calls the "update_items"
    // method to update the list.

    let mut items: Vec<types::SearchItem> = Vec::new();

    extern "C" fn callback(
        query: *const c_char,
        app: *const c_void,
    ) {
        let query = unsafe {CStr::from_ptr(query)};
        println!("{}", query.to_string_lossy());
    };

    unsafe {
        crate::interop::interop_show_search(
            metadata,
            callback,
            &mut items as *mut Vec<types::SearchItem> as *mut c_void,
        );
    }

    // value_map

    None
}
