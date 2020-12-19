use super::Rules;
use std::collections::{HashMap, HashSet, VecDeque};

struct ParsedRule {
    id: usize,
    leaves_used: HashSet<usize>,
    internal_rules: Vec<String>,
}

fn is_leaf(raw_rule: &str) -> bool {
    raw_rule.contains("\"")
}

fn parse_leaf<'a>(raw_rule: &'a str) -> Result<(usize, String), ()> {
    let rule_parts: Vec<&str> = raw_rule.split(":").collect();
    if rule_parts.len() != 2 {
        println!(
            "Invalid rule - got {} parts instead of 2 after parsing on ':'",
            rule_parts.len()
        );
        return Err(());
    }
    let r_match = rule_parts[1].trim();
    Ok((
        rule_parts[0].trim().parse::<usize>().unwrap(),
        r_match.replace("\"", ""),
    ))
}

fn parse_rule(raw_rule: &str) -> Result<ParsedRule, ()> {
    let rule_parts: Vec<&str> = raw_rule.split(":").collect();
    if rule_parts.len() != 2 {
        println!(
            "Invalid rule - got {} parts instead of 2 after parsing on ':'",
            rule_parts.len()
        );
        return Err(());
    }
    let rule_id = rule_parts[0].trim().parse::<usize>().unwrap();
    let internal_rules = rule_parts[1]
        .split("|")
        .map(|s| String::from(s.trim()))
        .collect::<Vec<String>>();
    let leaves_used = internal_rules
        .iter()
        .fold(HashSet::<usize>::new(), |mut acc, s| {
            s.split_whitespace().for_each(|d| {
                let _ = acc.insert(d.parse::<usize>().unwrap());
            });
            acc
        });
    Ok(ParsedRule {
        id: rule_id,
        leaves_used: leaves_used,
        internal_rules: internal_rules,
    })
}

pub fn parse_rules(raw_rules: Vec<String>, infinite_rules: Option<HashSet<usize>>) -> Rules {
    let mut leaves: HashSet<usize> = HashSet::new();
    let mut leaves_match: HashMap<usize, String> = HashMap::new();
    let mut rules_to_parse: VecDeque<ParsedRule> = VecDeque::new();
    for raw_rule in raw_rules.iter() {
        if is_leaf(raw_rule.as_str()) {
            let (leaf_id, leaf_marker) = parse_leaf(raw_rule).unwrap();
            leaves.insert(leaf_id);
            leaves_match.insert(leaf_id, leaf_marker);
        } else {
            rules_to_parse.push_back(parse_rule(raw_rule).unwrap());
        }
    }
    // println!("Found leaves {:?}", leaves);
    for leave in leaves_match.iter() {
        println!("> {} -> {}", leave.0, leave.1);
    }
    // println!("Inserting rules...");
    loop {
        if rules_to_parse.is_empty() {
            break;
        }
        let rule = rules_to_parse.pop_back().unwrap();
        let mut contains_infinite_rule = false;
        if rule.leaves_used.iter().all(|id| leaves.contains(id)) {
            if infinite_rules.is_some() && infinite_rules.clone().unwrap().contains(&rule.id) {
                contains_infinite_rule = true;
            }
            if !contains_infinite_rule || (contains_infinite_rule && rule.id == 8) {
                // Create the regex
                let items: Vec<String> = rule
                    .internal_rules
                    .iter()
                    .map(|s| {
                        s.split_whitespace()
                            .map(|s_id| {
                                let id = s_id.parse::<usize>().unwrap();
                                leaves_match.get(&id).unwrap().clone()
                            })
                            .collect::<String>()
                    })
                    .collect();
                let mut re = format!("(:?{})", items.join("|"));
                leaves.insert(rule.id);
                if contains_infinite_rule && rule.id == 8 {
                    re.push('+');
                }
                leaves_match.insert(rule.id, re.clone());
            } else if rule.id == 11 {
                // HARDCODED RULES! WHO CARES?!
                // UGLY THINGS... DO NOT TRY THAT AT HOME PLEASE...
                // I FEEL LIKE A TERRIBLE PROGRAMMER NOW -_-
                let left_part = leaves_match.get(&42).unwrap().clone();
                let right_part = leaves_match.get(&31).unwrap().clone();
                let mut re = String::from("(:?");
                for i in 0..13 {
                    if i == 0 {
                        re.push_str(format!("(:?{}{})", left_part, right_part).as_str());
                        continue;
                    }
                    re.push('|');
                    re.push_str(
                        format!("(:?{}{{{}}}{}{{{}}})", left_part, i + 1, right_part, i + 1)
                            .as_str(),
                    );
                }
                re.push(')');
                leaves.insert(rule.id);
                leaves_match.insert(rule.id, re.clone());
                // println!("RULE IS {}", re);
            }

            continue;
        }
        // Otherwise, put it to the front for the next tour
        rules_to_parse.push_front(rule);
    }
    Rules { map: leaves_match }
}

pub fn find_proposal_index(v: &Vec<String>) -> usize {
    for (index, s) in v.iter().enumerate() {
        if !s.contains(":") {
            return index;
        }
    }
    v.len()
}
