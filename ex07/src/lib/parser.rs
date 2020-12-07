use std::collections::{HashMap};
use regex::Regex;

use super::rules::{BagRule, Rule, Rules};

fn parse_bag_rule<'a>(bag_rule: &'a str) -> Option<Vec<BagRule>> {
    lazy_static! {
        static ref NO_OTHER_BAG_REGEX: Regex = Regex::new(r"no other bag[s]?*").unwrap();
    }
    let bag_rule_match: Vec<&str> = bag_rule.split(",").collect();
    if bag_rule_match.len() == 0 {
        return None;
    }
    let mut bag_rules: Vec<&str> = bag_rule_match.into_iter().map(|s| s.trim()).collect();
    let bag_rules_len = bag_rules.len();
    // The last bag rule may contains a '.' character
    if let Some(rule_without_punc) = bag_rules[bag_rules_len - 1].strip_suffix(".") {
        bag_rules[bag_rules_len - 1] = rule_without_punc;
    }
    // Trick in order to not return a None response
    if bag_rules_len == 1 && NO_OTHER_BAG_REGEX.is_match(bag_rules[0]) {
        return Some(vec![(String::from(""), 0)]);
    }
    let v = bag_rules.iter().fold(Vec::new(), |mut vector, s| {
        let mut bag_description_iter = (*s).chars();
        let nb_bags: u32 = bag_description_iter
            .by_ref()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let bag_type =
            String::from(get_bag_name(bag_description_iter.collect::<String>().as_str()).unwrap());
        vector.push((bag_type, nb_bags));
        vector
    });
    Some(v)
}

fn parse_rule<'a>(rule: &'a str) -> Option<Rule> {
    let rule_match: Vec<&str> = rule.split("contain").collect();
    if rule_match.len() != 2 {
        println!("Did not found any match in {}", rule);
        return None;
    }
    let bag_name = rule_match[0].trim();
    if let Some(bag_rule) = parse_bag_rule(rule_match[1].trim()) {
        return Some(Rule(
            String::from(get_bag_name(bag_name).unwrap()),
            bag_rule,
        ));
    };
    return None;
}

fn get_bag_name<'a>(full_bag_description: &'a str) -> Result<&'a str, ()> {
    lazy_static! {
        static ref BAG_NAME_REGEX: Regex = Regex::new(r"(?i)([\w ]+) bag[s]?").unwrap();
    }
    if let Some(captures) = BAG_NAME_REGEX.captures(full_bag_description) {
        return Ok(captures.get(1).map_or("", |m| m.as_str()));
    }
    println!("Cannot extract the bag name from {}", full_bag_description);
    Err(())
}

pub fn parse_file(file_content: Vec<String>) -> Result<Rules, ()> {
    let mut hash_map = HashMap::new();
    for raw_bag_rule in file_content.iter() {
        if raw_bag_rule.trim().is_empty() {
            continue;
        }
        if let Some(bag_rule) = parse_rule(raw_bag_rule) {
            let Rule(bag_name, rule) = bag_rule;
            if hash_map.contains_key(&bag_name) {
                println!("Error: Duplicate entry {}", bag_name);
                return Err(());
            }
            hash_map.insert(bag_name, rule);
        }
    }
    Ok(Rules::new(hash_map))
}
