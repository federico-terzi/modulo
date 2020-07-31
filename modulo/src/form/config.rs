use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

fn default_title() -> String {
    "modulo".to_owned()
}

fn default_icon() -> Option<String> {
    None
}

fn default_fields() -> HashMap<String, FieldConfig> {
    HashMap::new()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormConfig {
    #[serde(default = "default_title")]
    pub title: String,

    #[serde(default = "default_icon")]
    pub icon: Option<String>,

    pub layout: String,

    #[serde(default = "default_fields")]
    pub fields: HashMap<String, FieldConfig>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FieldConfig {
    pub field_type: FieldTypeConfig,
}

impl Default for FieldConfig {
    fn default() -> Self {
        Self {
            field_type: FieldTypeConfig::Text(TextFieldConfig {
                ..Default::default()
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FieldTypeConfig {
    Text(TextFieldConfig),
    Choice(ChoiceFieldConfig),
    List(ListFieldConfig),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextFieldConfig {
    pub default: String,
    pub multiline: bool,
}

impl Default for TextFieldConfig {
    fn default() -> Self {
        Self {
            default: "".to_owned(),
            multiline: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChoiceFieldConfig {
    pub values: Vec<String>,
}

impl Default for ChoiceFieldConfig {
    fn default() -> Self {
        Self { values: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListFieldConfig {
    pub values: Vec<String>,
}

impl Default for ListFieldConfig {
    fn default() -> Self {
        Self { values: Vec::new() }
    }
}

impl<'de> serde::Deserialize<'de> for FieldConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let auto_field = AutoFieldConfig::deserialize(deserializer)?;
        Ok(FieldConfig::from(&auto_field))
    }
}

impl<'a> From<&'a AutoFieldConfig> for FieldConfig {
    fn from(other: &'a AutoFieldConfig) -> Self {
        let field_type = match other.field_type.as_ref() {
            "text" => {
                let mut config: TextFieldConfig = Default::default();

                if let Some(default) = &other.default {
                    config.default = default.clone();
                }

                config.multiline = other.multiline;

                FieldTypeConfig::Text(config)
            }
            "choice" => {
                let config = ChoiceFieldConfig {
                    values: other.values.clone(),
                    ..Default::default()
                };

                FieldTypeConfig::Choice(config)
            }
            "list" => {
                let config = ListFieldConfig {
                    values: other.values.clone(),
                    ..Default::default()
                };

                FieldTypeConfig::List(config)
            }
            _ => {
                panic!("invalid field type: {}", other.field_type);
            }
        };

        Self { field_type }
    }
}

fn default_type() -> String {
    "text".to_owned()
}

fn default_default() -> Option<String> {
    None
}

fn default_multiline() -> bool {
    false
}

fn default_values() -> Vec<String> {
    Vec::new()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoFieldConfig {
    #[serde(rename = "type", default = "default_type")]
    pub field_type: String,

    #[serde(default = "default_default")]
    pub default: Option<String>,

    #[serde(default = "default_multiline")]
    pub multiline: bool,

    #[serde(default = "default_values")]
    pub values: Vec<String>,
}
