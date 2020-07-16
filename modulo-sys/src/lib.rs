#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod form;

pub fn show_window() {
    unsafe {
        let string = std::ffi::CString::new("test bindings").unwrap();;
    
        let metadata = FormMetadata {
            text: string.as_ptr(),
        };

        crate::form::show_window(&metadata);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
