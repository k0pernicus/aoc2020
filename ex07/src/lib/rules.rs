use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

pub type BagRule = (String, u32);
pub struct Rule(pub String, pub Vec<BagRule>);

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} -> {:?})", self.0, self.1)
    }
}

pub struct Rules {
    rules: HashMap<String, Vec<BagRule>>,
    predecessor_tree: HashMap<String, Vec<String>>,
}

impl Rules {
    pub fn new(rules: HashMap<String, Vec<BagRule>>) -> Rules {
        let mut r = Rules {
            rules,
            predecessor_tree: HashMap::new(),
        };
        r.build_predecessors_tree();
        r
    }
    pub fn compute_sum_contained_nb_bags(&self, bag_name: &str) -> u32 {
        if bag_name.is_empty() {
            return 0;
        }
        if let Some(bag_rule) = self.rules.get(bag_name) {
            return bag_rule.iter().fold(1u32, |sum, child_bag| {
                sum + (child_bag.1
                    * u32::max(
                        1u32,
                        self.compute_sum_contained_nb_bags(child_bag.0.as_str()),
                    ))
            });
        }
        1u32
    }
    fn build_predecessors_tree(&mut self) {
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