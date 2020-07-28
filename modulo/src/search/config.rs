use serde::{Deserialize, Deserializer, Serialize};

fn default_title() -> String {
    "modulo".to_owned()
}

fn default_items() -> Vec<SearchItem> {
    Vec::new()
}

fn default_algorithm() -> String {
    "iexact".to_owned()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchConfig {
    #[serde(default = "default_title")]
    pub title: String,

    #[serde(default = "default_items")]
    pub items: Vec<SearchItem>,

    #[serde(default = "default_algorithm")]
    pub algorithm: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchItem {
    pub id: String,
    pub label: String,
}