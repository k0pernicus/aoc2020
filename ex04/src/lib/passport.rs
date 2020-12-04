use itertools::Itertools;
use regex::Regex;
use std::default;
use std::str;

#[derive(Debug)]
pub struct Passport {
    id: Option<String>,
    birth_year: Option<String>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    /// Check that all entries, except countryID, are set
    pub fn is_valid(&self) -> bool {
        let is_valid = self.id.is_some()
            && self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some();
        if is_valid {
            println!("{:?} is valid", self);
        }
        return is_valid;
    }
}

impl default::Default for Passport {
    fn default() -> Self {
        Passport {
            id: None,
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            country_id: None,
        }
    }
}

fn get_birth_year(birth_year: &str) -> Option<String> {
    if birth_year.len() != 4 {
        return None;
    }
    match birth_year.parse::<i32>() {
        Ok(birth_year_i) => {
            if birth_year_i >= 1920 && birth_year_i <= 2002 {
                return Some(String::from(birth_year));
            }
            return None;
        }
        _ => return None,
    }
}

fn get_issue_year(issue_year: &str) -> Option<u32> {
    if issue_year.len() != 4 {
        return None;
    }
    match issue_year.parse::<i32>() {
        Ok(issue_year_i) => {
            if issue_year_i >= 2010 && issue_year_i <= 2020 {
                return Some(issue_year_i as u32);
            }
            return None;
        }
        _ => return None,
    }
}

fn get_expiration_year(expiration_year: &str) -> Option<u32> {
    if expiration_year.len() != 4 {
        return None;
    }
    match expiration_year.parse::<i32>() {
        Ok(expiration_year_i) => {
            if expiration_year_i >= 2020 && expiration_year_i <= 2030 {
                return Some(expiration_year_i as u32);
            }
            return None;
        }
        _ => return None,
    }
}

fn get_height(height: &str) -> Option<String> {
    let mut height_iter = height.chars();
    let height_nb = height_iter
        .take_while_ref(|x| !x.is_alphabetic())
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0);
    let height_unit = height_iter.collect::<String>();
    match height_unit.to_lowercase().as_str() {
        "cm" => {
            if height_nb >= 150 && height_nb <= 193 {
                return Some(String::from(height));
            }
            return None;
        }
        "in" => {
            if height_nb >= 59 && height_nb <= 76 {
                return Some(String::from(height));
            }
            return None;
        }
        _ => return None,
    }
}

fn get_hair_color(hair_color: &str) -> Option<String> {
    let re = Regex::new(r"#[[:alpha:]0-9]{6}").unwrap();
    if re.is_match(hair_color) {
        return Some(String::from(hair_color));
    }
    return None;
}

fn get_eye_color(eye_color: &str) -> Option<String> {
    if eye_color.trim().len() == 0 {
        return None;
    }
    match eye_color {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(String::from(eye_color)),
        _ => None,
    }
}

fn get_passport_id(passport_id: &str) -> Option<String> {
    if passport_id.len() != 9 {
        return None;
    }
    if passport_id
        .chars()
        .filter_map(|c| {
            if !c.is_digit(10) {
                return Some(true);
            }
            return None;
        })
        .count()
        == 0
    {
        return Some(String::from(passport_id));
    }
    return None;
}

impl str::FromStr for Passport {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_str = s.trim();
        let entries = parsed_str.split_whitespace();

        let mut passport = Passport::default();

        for entry in entries {
            let split_entry: Vec<&str> = entry.split(":").collect();
            if split_entry.len() != 2 {
                println!("Split entry ({}) has more than two fields...", entry);
                return Err("Split entry has more than two fields");
            }
            let (entry_name, entry_value) = (split_entry[0], split_entry[1]);
            match entry_name {
                "byr" => passport.birth_year = get_birth_year(entry_value),
                "iyr" => passport.issue_year = get_issue_year(entry_value),
                "eyr" => passport.expiration_year = get_expiration_year(entry_value),
                "hgt" => passport.height = get_height(entry_value),
                "hcl" => passport.hair_color = get_hair_color(entry_value),
                "ecl" => passport.eye_color = get_eye_color(entry_value),
                "pid" => passport.id = get_passport_id(entry_value),
                "cid" => passport.country_id = Some(String::from(entry_value)),
                _ => println!("Unknown entry {}", entry_name),
            }
        }

        Ok(passport)
    }
}
