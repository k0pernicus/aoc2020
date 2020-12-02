use std::collections::BTreeMap;

type Range = (u32, u32);

fn parse_password_policy<'a>(s: &'a str) -> Option<(Range, char)> {
    let sanitized_s = s.trim_end();
    let s_split = s.split_ascii_whitespace().collect::<Vec<&str>>();
    if s_split.len() != 2 {
        println!("Found more than two patterns in {}...", s);
        return None;
    }
    let range = s_split[0]
        .split("-")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let char: char = s_split[1].trim_start().chars().collect::<Vec<char>>()[0];
    return Some(((range[0], range[1]), char));
}

fn get_char_occurences_in_str<'a>(s: &'a str) -> BTreeMap<char, u32> {
    let mut count: BTreeMap<char, u32> = BTreeMap::new();
    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    return count;
}

fn get_char_indexes_in_str<'a>(s: &'a str) -> BTreeMap<char, Vec<usize>> {
    let mut indexes: BTreeMap<char, Vec<usize>> = BTreeMap::new();
    for (index, c) in s.chars().enumerate() {
        indexes.entry(c).or_insert(Vec::new()).push(index);
    }
    return indexes;
}

pub fn compute_nb_of_valid_passwords_part_1(v: &Vec<String>) -> u32 {
    if v.len() == 0 {
        return 0u32;
    }
    let mut valid_passwords: u32 = 0;
    for l in v.iter() {
        let l_split = l.split(":").collect::<Vec<&str>>();
        if l_split.len() != 2 {
            println!(
                "Found weird statement for line {}: multiple ':' characters found...",
                l
            );
            continue;
        }
        if let Some((password_policy_range, password_policy_char)) =
            parse_password_policy(l_split[0])
        {
            let char_occurences: BTreeMap<char, u32> = get_char_occurences_in_str(l_split[1]);
            if !char_occurences.contains_key(&password_policy_char) {
                continue;
            }
            let char_occurence = *char_occurences.get(&password_policy_char).unwrap();
            if char_occurence < password_policy_range.0 || char_occurence > password_policy_range.1
            {
                continue;
            }
            valid_passwords += 1;
        }
    }
    return valid_passwords;
}

pub fn compute_nb_of_valid_passwords_part_2(v: &Vec<String>) -> u32 {
    if v.len() == 0 {
        return 0u32;
    }
    let mut valid_passwords: u32 = 0;
    for l in v.iter() {
        let l_split = l.split(":").collect::<Vec<&str>>();
        if l_split.len() != 2 {
            println!(
                "Found weird statement for line {}: multiple ':' characters found...",
                l
            );
            continue;
        }
        if let Some((password_policy_range, password_policy_char)) =
            parse_password_policy(l_split[0])
        {
            let char_indexes: BTreeMap<char, Vec<usize>> = get_char_indexes_in_str(l_split[1]);
            if !char_indexes.contains_key(&password_policy_char) {
                println!("{} has not been found in the map", password_policy_char);
                continue;
            }
            let c_char_indexes = char_indexes.get(&password_policy_char).unwrap();
            if (!c_char_indexes.contains(&(password_policy_range.0 as usize))
                && !c_char_indexes.contains(&(password_policy_range.1 as usize)))
                || (c_char_indexes.contains(&(password_policy_range.0 as usize))
                    && c_char_indexes.contains(&(password_policy_range.1 as usize)))
            {
                continue;
            }
            valid_passwords += 1;
        }
    }
    return valid_passwords;
}
