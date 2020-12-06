use std::collections::{HashMap, HashSet};

pub fn count_questions_with_yes_response(responses: &str) -> u32 {
    if responses.len() == 0 {
        return 0;
    }
    let chars = responses.chars();
    let mut hash_set = HashSet::new();
    chars.for_each(|c| {
        let _ = hash_set.insert(c);
    });
    hash_set.len() as u32
}

pub fn count_questions_with_duplicate_yes_reponse(responses: &str, nb_persons: u32) -> u32 {
    if responses.len() == 0 {
        return 0;
    }
    let chars = responses.chars();
    let mut hash_map = HashMap::new();
    chars.for_each(|c| *hash_map.entry(c).or_insert(0) += 1);
    hash_map.retain(|_, &mut v| v >= nb_persons);
    hash_map.len() as u32
}
