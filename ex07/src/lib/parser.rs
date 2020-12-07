use std::collections::{HashMap, HashSet, VecDeque};
use std::default::Default;
use std::fmt;

use regex::Regex;

type BagRule = (String, u32);
struct Rule(String, Vec<BagRule>);

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} -> {:?})", self.0, self.1)
    }
}

pub struct Rules {
    pub rules: HashMap<String, Vec<BagRule>>,
    predecessor_tree: HashMap<String, Vec<String>>,
}

impl Rules {
    pub fn new(rules: HashMap<String, Vec<BagRule>>) -> Rules {
        Rules {
            rules,
            predecessor_tree: HashMap::new(),
        }
    }
    pub fn compute_sum_contained_nb_bags(&self, bag_name: &str) -> u32 {
        if bag_name.is_empty() {
            return 0;
        }
        if let Some(bag_rule) = self.rules.get(bag_name) {
            return bag_rule.iter().fold(0u32, |sum, child_bag| {
                sum + (child_bag.1
                    * u32::max(
                        1u32,
                        self.compute_sum_contained_nb_bags(child_bag.0.as_str()),
                    ))
            });
        }
        0u32
    }
    pub fn build_predecessors_tree(&mut self) {
        self.predecessor_tree.clear();
        for rule in self.rules.iter() {
            let (c_bag_name, c_bag_rules) = rule;
            if c_bag_rules.len() == 0 {
                continue;
            }
            for (c_bag_rule_name, _) in c_bag_rules.iter() {
                if c_bag_rule_name.trim().is_empty() {
                    continue;
                }
                self.predecessor_tree
                    .entry(c_bag_rule_name.clone())
                    .or_insert(Vec::new())
                    .push(c_bag_name.clone());
            }
        }
    }
    pub fn find_parent_bags(&self, bag_name: &str) -> HashSet<String> {
        let mut queue = VecDeque::new();
        queue.push_back(bag_name);
        let mut visited = HashSet::new();
        visited.insert(bag_name);
        let mut parent_bags = HashSet::new();
        while !queue.is_empty() {
            let c_bag_name = queue.pop_back().unwrap();
            println!("Visiting {}", c_bag_name);
            if c_bag_name != bag_name {
                parent_bags.insert(String::from(c_bag_name));
            }
            if let Some(predecessors) = self.predecessor_tree.get(c_bag_name) {
                for predecessor in predecessors.iter() {
                    if visited.contains(predecessor.as_str()) {
                        continue;
                    }
                    visited.insert(predecessor.as_str());
                    queue.push_back(predecessor);
                }
            }
        }
        parent_bags
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            rules: HashMap::new(),
            predecessor_tree: HashMap::new(),
        }
    }
}

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
        println!("Parsing rule '{}'", raw_bag_rule);
        if let Some(bag_rule) = parse_rule(raw_bag_rule) {
            let Rule(bag_name, rule) = bag_rule;
            if hash_map.contains_key(&bag_name) {
                println!("Error: Duplicate entry {}", bag_name);
                return Err(());
            }
            println!("'{} -> {:?}'", bag_name, rule);
            hash_map.insert(bag_name, rule);
        }
    }
    Ok(Rules::new(hash_map))
}
