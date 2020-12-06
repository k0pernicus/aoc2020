pub type Group = Vec<String>;

pub fn parse_groups(lines: Vec<String>) -> Vec<Group> {
    let mut answers: Vec<Vec<String>> = Vec::new();
    answers.push(Vec::new());
    let mut c_len = 0;
    for answer in lines {
        if answer.is_empty() {
            answers.push(Vec::new());
            c_len += 1;
        } else {
            // Cannot use last as it consumes the last element
            answers[c_len].push(answer);
        }
    }
    answers
}
