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
                field_type: FieldType::Row(RowMetadata {
                    fields
                }),
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

        pub fn text(id: &str, default_text: &str) -> Self {
            Self {
                id: Some(id.to_owned()),
                field_type: FieldType::Text(TextMetadata {
                    default_text: default_text.to_owned(),
                }),
            }
        }
    }

    pub enum FieldType {
        Row(RowMetadata),
        Label(LabelMetadata),
        Text(TextMetadata),
    }

    pub struct RowMetadata {
        pub fields: Vec<Field>,
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

    pub(crate) trait Interoperable {
        fn as_ptr(&self) -> *const c_void;
    }

    pub(crate) struct OwnedForm {
        title: CString,
        fields: Vec<OwnedField>,

        _metadata: Vec<FieldMetadata>,
        _interop: Box<FormMetadata>,
    }

    impl Interoperable for OwnedForm {
        fn as_ptr(&self) -> *const c_void {
            &(*self._interop) as *const FormMetadata as *const c_void
        }
    }

    impl From<types::Form> for OwnedForm {
        fn from(form: types::Form) -> Self {
            let title = CString::new(form.title).expect("unable to convert form title to CString");
            let fields: Vec<OwnedField> = form.fields.into_iter().map(|field| {
                field.into()
            }).collect();

            let _metadata: Vec<FieldMetadata> = fields.iter().map(|field| {
                field.metadata()
            }).collect();

            let _interop = Box::new(FormMetadata {
                windowTitle: title.as_ptr(),
                fields: _metadata.as_ptr(),
                fieldSize: fields.len() as c_int,
            });

            Self {
                title,
                fields,
                _metadata,
                _interop,
            }
        }
    }

    struct OwnedField {
        id: Option<CString>,
        field_type: FieldType,
        specific: Box<dyn Interoperable>,
    }

    impl From<types::Field> for OwnedField {
        fn from(field: types::Field) -> Self {
            let id = if let Some(id) = field.id {
                Some(CString::new(id).expect("unable to create cstring for field id"))
            }else{
                None
            };

            let field_type = match field.field_type {
                types::FieldType::Row(_) => {
                    FieldType_ROW
                }
                types::FieldType::Label(_) => {
                    FieldType_LABEL
                },
                types::FieldType::Text(_) => {
                    FieldType_TEXT
                }
            };

            // TODO: clean up this match
            let specific: Box<dyn Interoperable> = match field.field_type {
                types::FieldType::Row(metadata) => {
                    let owned_metadata: OwnedRowMetadata = metadata.into();
                    Box::new(owned_metadata)
                }
                types::FieldType::Label(metadata) => {
                    let owned_metadata: OwnedLabelMetadata = metadata.into();
                    Box::new(owned_metadata)
                }
                types::FieldType::Text(metadata) => {
                    let owned_metadata: OwnedTextMetadata = metadata.into();
                    Box::new(owned_metadata)
                }
            };

            Self {
                id,
                field_type,
                specific,
            }
        }
    }

    impl OwnedField {
        pub fn metadata(&self) -> FieldMetadata {
            let id_ptr = if let Some(id) = self.id.as_ref() {
                id.as_ptr()
            }else{
                null()
            };

            FieldMetadata {
                id: id_ptr,
                fieldType: self.field_type,
                specific: self.specific.as_ptr(),
            }
        }
    }

    struct OwnedLabelMetadata {
        text: CString,
        _interop: Box<LabelMetadata>,
    }

    impl Interoperable for OwnedLabelMetadata {
        fn as_ptr(&self) -> *const c_void {
            &(*self._interop) as *const LabelMetadata as *const c_void
        }
    }

    impl From<types::LabelMetadata> for OwnedLabelMetadata {
        fn from(label_metadata: types::LabelMetadata) -> Self {
            let text = CString::new(label_metadata.text).expect("unable to convert label text to CString");
            let _interop = Box::new(LabelMetadata {
                text: text.as_ptr(),
            });
            Self {
                text,
                _interop,
            }
        }
    }

    struct OwnedTextMetadata {
        default_text: CString,
        _interop: Box<TextMetadata>,
    }

    impl Interoperable for OwnedTextMetadata {
        fn as_ptr(&self) -> *const c_void {
            &(*self._interop) as *const TextMetadata as *const c_void
        }
    }

    impl From<types::TextMetadata> for OwnedTextMetadata {
        fn from(text_metadata: types::TextMetadata) -> Self {
            let default_text = CString::new(text_metadata.default_text).expect("unable to convert default text to CString");
            let _interop = Box::new(TextMetadata {
                defaultText: default_text.as_ptr(),
            });
            Self {
                default_text,
                _interop,
            }
        }
    }

    struct OwnedRowMetadata {
        fields: Vec<OwnedField>,

        _metadata: Vec<FieldMetadata>,
        _interop: Box<RowMetadata>,
    }

    impl Interoperable for OwnedRowMetadata {
        fn as_ptr(&self) -> *const c_void {
            &(*self._interop) as *const RowMetadata as *const c_void
        }
    }

    impl From<types::RowMetadata> for OwnedRowMetadata {
        fn from(row_metadata: types::RowMetadata) -> Self {
            let fields: Vec<OwnedField> = row_metadata.fields.into_iter().map(|field| {
                field.into()
            }).collect();

            let _metadata: Vec<FieldMetadata> = fields.iter().map(|field| {
                field.metadata()
            }).collect();

            let _interop = Box::new(RowMetadata {
                fields: _metadata.as_ptr(),
                fieldSize: _metadata.len() as c_int,
            });

            Self {
                fields,
                _metadata,
                _interop,
            }
        }
    }
}

pub fn show(form: types::Form) {
    use interop::Interoperable;
    
    unsafe {
        let owned_form: interop::OwnedForm = form.into();
        let metadata: *const FormMetadata = owned_form.as_ptr() as *const FormMetadata;

        // TODO: Nested rows should fail, add check

        interop_show_form(metadata);
    }
}