use super::parser::layout::Token;
use modulo_sys::form::types::*;

fn create_field(token: &Token) -> Field {
    match token {
        Token::Text(text) => {
            Field {
                field_type: FieldType::Label(LabelMetadata {
                    text: text.clone(),
                }),
                ..Default::default()
            }
        }
        Token::Field(name) => {
            // TODO: create field type based on name configs
            Field {
                id: Some(name.clone()),
                field_type: FieldType::Text(TextMetadata {
                    default_text: "".to_owned(),
                }),
                ..Default::default()
            }
        }
    }
}

pub fn generate(structure: Vec<Vec<Token>>) -> Form {
    let mut fields = Vec::new();

    for row in structure.iter() {
        let current_field = if row.len() == 1 { // Single field
            create_field(&row[0])
        }else{ // Row field 
            let inner_fields = row.iter().map(|token| {
                create_field(token)
            }).collect();

            Field {
                field_type: FieldType::Row(RowMetadata {
                    fields: inner_fields,
                }),
                ..Default::default()
            }            
        };

        fields.push(current_field)
    }

    println!("{:?}", fields);

    Form {
        title: "modulo".to_owned(), // TODO:change
        fields,
    }
}