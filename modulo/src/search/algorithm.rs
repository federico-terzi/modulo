use modulo_sys::search::types::SearchItem;

pub fn get_algorithm(name: &str) -> Box<dyn Fn(&str, &[SearchItem]) -> Vec<usize>> {
    match name {
        "exact" => Box::new(exact_match),
        "iexact" => Box::new(case_insensitive_exact_match),
        "ikey" => Box::new(case_insensitive_keyword),
        _ => panic!("unknown search algorithm: {}", name),
    }
}

fn exact_match(query: &str, items: &[SearchItem]) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            item.label.contains(query) || item.trigger.as_deref().map_or(false, |t| t.contains(query))
        })
        .map(|(i, _)| i)
        .collect()
}

fn case_insensitive_exact_match(query: &str, items: &[SearchItem]) -> Vec<usize> {
    let lowercase_query = query.to_lowercase();
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            item.label.to_lowercase().contains(&lowercase_query)
                || item
                    .trigger.as_deref()
                    .map_or(false, |t| t.to_lowercase().contains(query))
        })
        .map(|(i, _)| i)
        .collect()
}

fn case_insensitive_keyword(query: &str, items: &[SearchItem]) -> Vec<usize> {
    let lowercase_query = query.to_lowercase();
    let keywords: Vec<&str> = lowercase_query.split_whitespace().collect();
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            for keyword in keywords.iter() {
                if !item.label.to_lowercase().contains(keyword)
                    && !item
                        .trigger.as_deref()
                        .map_or(false, |t| t.to_lowercase().contains(keyword))
                {
                    return false;
                }
            }

            true
        })
        .map(|(i, _)| i)
        .collect()
}
