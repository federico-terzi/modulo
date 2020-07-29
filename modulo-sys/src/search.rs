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
        items: Vec<OwnedSearchItem>,
        pub(crate) interop_items: Vec<SearchItem>,
        _interop: Box<SearchMetadata>,
    }

    impl Interoperable for OwnedSearch {
        fn as_ptr(&self) -> *const c_void {
            &(*self._interop) as *const SearchMetadata as *const c_void
        }
    }

    impl From<&types::Search> for OwnedSearch {
        fn from(search: &types::Search) -> Self {
            let title = CString::new(search.title.clone()).expect("unable to convert search title to CString");

            let items: Vec<OwnedSearchItem> = search.items.iter().map(|item| {
                item.into()
            }).collect();

            let interop_items: Vec<SearchItem> = items.iter().map(|item| {
                item.to_search_item()
            }).collect();

            let _interop = Box::new(SearchMetadata {
                windowTitle: title.as_ptr(),
            });

            Self {
                title,
                items,
                interop_items,
                _interop,
            }
        }
    }

    pub(crate) struct OwnedSearchItem {
        id: CString,
        label: CString,
    }

    impl OwnedSearchItem {
        fn to_search_item(&self) -> SearchItem {
            SearchItem {
                id: self.id.as_ptr(),
                label: self.label.as_ptr(),
            }
        }
    }

    impl From<&types::SearchItem> for OwnedSearchItem {
        fn from(item: &types::SearchItem) -> Self {
            let id = CString::new(item.id.clone()).expect("unable to convert item id to CString");
            let label = CString::new(item.label.clone()).expect("unable to convert item label to CString");

            Self {
                id,
                label,
            }
        }
    }
}

struct SearchData {
    owned_search: interop::OwnedSearch,
    items: Vec<types::SearchItem>,
    algorithm: Box<dyn Fn(&str, &Vec<types::SearchItem>)->Vec<usize>>,
}

pub fn show(search: types::Search, algorithm: Box<dyn Fn(&str, &Vec<types::SearchItem>)->Vec<usize>>) -> Option<String> {
    use crate::interop::SearchMetadata;
    use crate::Interoperable;

    let owned_search: interop::OwnedSearch = (&search).into();
    let metadata: *const SearchMetadata = owned_search.as_ptr() as *const SearchMetadata;

    let search_data = SearchData {
        owned_search,
        items: search.items,
        algorithm,
    };

    extern "C" fn search_callback(
        query: *const c_char,
        app: *const c_void,
        data: *const c_void,
    ) {
        let query = unsafe {CStr::from_ptr(query)};
        let query = query.to_string_lossy().to_string();

        let search_data = data as *const SearchData;
        let search_data = unsafe {&*search_data};

        let indexes = (*search_data.algorithm)(&query, &search_data.items);
        let items: Vec<crate::interop::SearchItem> = indexes.into_iter().map(|index| {
            search_data.owned_search.interop_items[index]
        }).collect();

        unsafe {
            crate::interop::update_items(app, items.as_ptr(), items.len() as c_int);
        }
    };

    let mut result: Option<String> = None;

    extern "C" fn result_callback(
        id: *const c_char,
        result: *mut c_void,
    ) {
        let id = unsafe {CStr::from_ptr(id)};
        let id = id.to_string_lossy().to_string();
        let result: *mut Option<String> = result as *mut Option<String>;
        unsafe {
            *result = Some(id);
        }
    }

    unsafe {
        crate::interop::interop_show_search(
            metadata,
            search_callback,
            &search_data as *const SearchData as *const c_void,
            result_callback,
            &mut result as *mut Option<String> as *mut c_void,
        );
    }

    println!("{:?}", result);

    result
}
