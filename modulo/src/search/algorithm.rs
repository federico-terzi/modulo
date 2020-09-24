use modulo_sys::search::types::SearchItem;

pub fn get_algorithm(name: &str) -> Box<dyn Fn(&str, &Vec<SearchItem>) -> Vec<usize>> {
    match name {
        "exact" => Box::new(exact_match),
        "iexact" => Box::new(case_insensitive_exact_match),
        "ikey" => Box::new(case_insensitive_keyword),
        _ => panic!("unknown search algorithm: {}", name),
    }
}

fn exact_match(query: &str, items: &Vec<SearchItem>) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.label.contains(query))
        .map(|(i, _)| i)
        .collect()
}

fn case_insensitive_exact_match(query: &str, items: &Vec<SearchItem>) -> Vec<usize> {
    let lowercase_query = query.to_lowercase();
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.label.to_lowercase().contains(&lowercase_query))
        .map(|(i, _)| i)
        .collect()
}

fn case_insensitive_keyword(query: &str, items: &Vec<SearchItem>) -> Vec<usize> {
    let lowercase_query = query.to_lowercase();
    let keywords: Vec<&str> = lowercase_query.split_whitespace().into_iter().collect();
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            for keyword in keywords.iter() {
                if !item.label.to_lowercase().contains(keyword) {
                    return false;
                }
            }

            true
        })
        .map(|(i, _)| i)
        .collect()
}
