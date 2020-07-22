use super::config::{FieldConfig, FormConfig};
use super::parser::layout::Token;
use crate::config::FieldTypeConfig;
use modulo_sys::form::types::*;
use std::collections::HashMap;

pub fn generate(config: FormConfig) -> Form {
    let structure = super::parser::layout::parse_layout(&config.layout);
    build_form(structure, config.fields)
}

fn create_field(token: &Token, field_map: &HashMap<String, FieldConfig>) -> Field {
    match token {
        Token::Text(text) => Field {
            field_type: FieldType::Label(LabelMetadata { text: text.clone() }),
            ..Default::default()
        },
        Token::Field(name) => {
            let config = if let Some(config) = field_map.get(name) {
                config.clone()
            } else {
                Default::default()
            };

            let field_type = match &config.field_type {
                FieldTypeConfig::Text(config) => FieldType::Text(TextMetadata {
                    default_text: config.default.clone(),
                    multiline: config.multiline,
                }),
                FieldTypeConfig::Choice(config) => FieldType::Choice(ChoiceMetadata {
                    values: config.values.clone(),
                    choice_type: ChoiceType::Dropdown,
                }),
                FieldTypeConfig::List(config) => FieldType::Choice(ChoiceMetadata {
                    values: config.values.clone(),
                    choice_type: ChoiceType::List,
                }),
            };

            Field {
                id: Some(name.clone()),
                field_type,
                ..Default::default()
            }
        }
    }
}

fn build_form(structure: Vec<Vec<Token>>, field_map: HashMap<String, FieldConfig>) -> Form {
    let mut fields = Vec::new();

    for row in structure.iter() {
        let current_field = if row.len() == 1 {
            // Single field
            create_field(&row[0], &field_map)
        } else {
            // Row field
            let inner_fields = row
                .iter()
                .map(|token| create_field(token, &field_map))
                .collect();

            Field {
                field_type: FieldType::Row(RowMetadata {
                    fields: inner_fields,
                }),
                ..Default::default()
            }
        };

        fields.push(current_field)
    }

    Form {
        title: "modulo".to_owned(), // TODO: change
        fields,
    }
}
