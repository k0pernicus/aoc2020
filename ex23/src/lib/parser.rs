use super::Cups;

#[derive(Debug)]
pub enum ParserError {
    NoInput,
    WrongType,
}

pub type ParserResult = Result<Cups, ParserError>;

pub fn get_cups(s: String, fill_to: Option<u32>) -> ParserResult {
    if s.len() == 0 {
        return Err(ParserError::NoInput);
    }
    let mut raw_cups: Vec<u32> = Vec::with_capacity(if fill_to.is_none() {
        s.len()
    } else {
        fill_to.unwrap() as usize
    });
    for c in s.chars() {
        match c.to_digit(10) {
            Some(digit) => raw_cups.push(digit),
            None => return Err(ParserError::WrongType),
        }
    }
    if fill_to.is_none() {
        return Ok(Cups::new(raw_cups));
    }
    let max_id = raw_cups.iter().max().unwrap();
    for i in *max_id..fill_to.unwrap() {
        raw_cups.push(i);
    }
    Ok(Cups::new(raw_cups))
}
