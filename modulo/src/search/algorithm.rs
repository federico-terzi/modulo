/*
 * This file is part of modulo.
 *
 * Copyright (C) 2020-2021 Federico Terzi
 *
 * modulo is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * modulo is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with modulo.  If not, see <https://www.gnu.org/licenses/>.
 */

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
            item.label.contains(query)
                || item.trigger.as_deref().map_or(false, |t| t.contains(query))
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
                    .trigger
                    .as_deref()
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
                        .trigger
                        .as_deref()
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
