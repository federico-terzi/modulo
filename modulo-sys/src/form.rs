use interop::FormMetadata;
use std::ffi::CString;
use std::ptr::null;

// Native bindings

#[allow(improper_ctypes)]
#[link(name = "modulosys", kind = "static")]
extern "C" {
    fn interop_show_form(metadata: *const FormMetadata);
}

// Form schema

pub mod types {
    pub struct Form {
        pub title: String,
        pub fields: Vec<Field>,
    }

    pub struct Field {
        pub id: Option<String>,
        pub field_type: FieldType,
    }

    impl Field {
        pub fn row(fields: Vec<Field>) -> Self {
            Self {
                id: None,
                field_type: FieldType::Row(fields),
            }
        }

        pub fn label(text: &str) -> Self {
            Self {
                id: None,
                field_type: FieldType::Label(LabelMetadata {
                    text: text.to_owned(),
                }),
            }
        }
    }

    pub enum FieldType {
        Row(Vec<Field>),
        Label(LabelMetadata),
        Text(TextMetadata),
    }

    pub struct LabelMetadata {
        pub text: String,
    }

    pub struct TextMetadata {
        pub default_text: String,
    }
}

// Form interop

mod interop {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    use std::ffi::{CString, c_void};
    use std::os::raw::c_int;
    use std::ptr::null;
    use super::types;

    trait Interoperable {
        fn as_ptr(&self) -> *const c_void;
    }

    struct OwnedForm {
        title: CString,
        fields: Vec<OwnedField>,
        fields_ptr: Vec<*const c_void>,
        interop: Box<FormMetadata>,
    }

    impl Interoperable for OwnedForm {
        fn as_ptr(&self) -> *const c_void {
            &(*self.interop) as *const FormMetadata as *const c_void
        }
    }

    impl From<types::Form> for OwnedForm {
        fn from(form: types::Form) -> Self {
            let title = CString::new(form.title).expect("unable to convert form title to CString");
            let fields: Vec<OwnedField> = form.fields.into_iter().map(|field| {
                field.into()
            }).collect();

            let fields_ptr: Vec<*const c_void> = fields.iter().map(|field| {
                field.as_ptr()
            }).collect();

            let interop = Box::new(FormMetadata {
                windowTitle: title.as_ptr(),
                fields: fields_ptr.as_ptr(),
                fieldSize: fields.len() as c_int,
            });

            Self {
                title,
                fields,
                fields_ptr,
                interop,
            }
        }
    }

    struct OwnedField {
        id: Option<CString>,
        field_type: FieldType,
        specific: Box<dyn Interoperable>,
        interop: Box<FieldMetadata>,
    }

    impl Interoperable for OwnedField {
        fn as_ptr(&self) -> *const c_void {
            &(*self.interop) as *const FieldMetadata as *const c_void
        }
    }

    impl From<types::Field> for OwnedField {
        fn from(field: types::Field) -> Self {
            let id = if let Some(id) = field.id {
                Some(CString::new(id).expect("unable to create cstring for field id"))
            }else{
                None
            };

            let field_type = match field.field_type {
                types::FieldType::Row(_) => {todo!()}
                types::FieldType::Label(_) => {
                    FieldType_LABEL
                },
                types::FieldType::Text(_) => {todo!()}
            };

            let id_ptr = if let Some(id) = id.as_ref() {
                id.as_ptr()
            }else{
                null()
            };

            let specific = match field.field_type {
                types::FieldType::Row(_) => {todo!()}
                types::FieldType::Label(metadata) => {
                    let owned_metadata: OwnedLabelMetadata = metadata.into();
                    Box::new(owned_metadata)
                }
                types::FieldType::Text(_) => {todo!()}
            };

            let interop = Box::new(FieldMetadata {
                id: id_ptr,
                fieldType: field_type,
                specific: &(*specific) as *const dyn Interoperable as *const c_void,
            });

            Self {
                id,
                field_type,
                specific,
                interop,
            }
        }
    }

    struct OwnedLabelMetadata {
        text: CString,
        interop: Box<LabelMetadata>,
    }

    impl Interoperable for OwnedLabelMetadata {
        fn as_ptr(&self) -> *const c_void {
            &(*self.interop) as *const LabelMetadata as *const c_void
        }
    }

    impl From<types::LabelMetadata> for OwnedLabelMetadata {
        fn from(label_metadata: types::LabelMetadata) -> Self {
            let text = CString::new(label_metadata.text).expect("unable to convert label text to CString");
            let interop = Box::new(LabelMetadata {
                text: text.as_ptr(),
            });
            Self {
                text,
                interop,
            }
        }
    }
}

pub fn show(form: types::Form) {
    unsafe {
        let title = CString::new(form.title).unwrap();
        
        

        // TODO: Nested rows should fail, add check
        let metadata = FormMetadata {
            windowTitle: title.as_ptr(),
            fields: null(),
            fieldSize: 0,
        };

        interop_show_form(&metadata);
    }
}