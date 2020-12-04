use super::passport::Passport;
use std::str::FromStr;

pub fn parse(lines: Vec<String>) -> Vec<Passport> {
    let mut c_line = 0;
    let nb_lines = lines.len();
    let mut passports = Vec::new();
    let mut raw_passport: String = String::from("");
    loop {
        if c_line >= nb_lines || lines[c_line].is_empty() {
            if let Ok(passport) = Passport::from_str(raw_passport.as_str()) {
                passports.push(passport);
            }
            raw_passport.clear();
        } else {
            raw_passport.push_str(" ");
            raw_passport.push_str(&lines[c_line]);
        }
        if c_line >= nb_lines {
            break;
        }
        c_line += 1;
    }
    passports
}
