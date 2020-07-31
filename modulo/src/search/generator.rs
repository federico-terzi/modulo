use crate::search::config::SearchConfig;
use modulo_sys::search::types;

pub fn generate(config: SearchConfig) -> types::Search {
    let items = config.items.into_iter().map(|item| {
        types::SearchItem {
            id: item.id,
            label: item.label
        }
    }).collect();

    types::Search {
        title: config.title,
        items,
        icon: config.icon,
    }
}