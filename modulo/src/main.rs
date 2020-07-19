use modulo_sys::form::types::*;

fn main() {
    println!("Hello, world!");
    modulo_sys::form::show(Form {
        title: "test title".to_owned(),
        fields: vec![
            Field {
                field_type: FieldType::Row(RowMetadata {
                    fields: vec![
                        Field {
                            field_type: FieldType::Label(LabelMetadata {
                                text: "Hey".to_owned(),
                            }),
                            weight: 1,
                            ..Default::default()
                        },
                        Field {
                            id: Some("name".to_owned()),
                            weight: 7,
                            field_type: FieldType::Text(TextMetadata {
                                default_text: "name".to_owned()
                            }),
                            ..Default::default()
                        }
                    ]
                }),
                ..Default::default()
            },
            Field {
                id: Some("message".to_owned()),
                field_type: FieldType::Text(TextMetadata {
                    default_text: "".to_owned()
                }),
                ..Default::default()
            }
        ],
    })
}
