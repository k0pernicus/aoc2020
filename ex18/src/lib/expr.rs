use std::collections::VecDeque;
use std::ops;

fn get_end_index_of_expression(s: &str) -> usize {
    let mut parenthesis_match: usize = 0;
    let mut index: usize = 0;
    for c in s.chars() {
        match c {
            '(' => {
                parenthesis_match += 1;
            }
            ')' => {
                parenthesis_match -= 1;
                if parenthesis_match <= 0 {
                    index += 1;
                    break;
                }
            }
            _ => {}
        }
        index += 1;
    }
    index
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn get_fn(&self) -> fn(isize, isize) -> isize {
        match self {
            Self::Add => ops::Add::add,
            Self::Mul => ops::Mul::mul,
        }
    }
    pub fn compute(&self, left: isize, right: isize) -> isize {
        let f = self.get_fn();
        f(left, right)
    }
}

pub fn evaluate_expression(s: &str, same_precedence_level: bool) -> isize {
    let mut acc = String::from("");
    let mut ops: VecDeque<Op> = VecDeque::new();
    let mut values: VecDeque<isize> = VecDeque::new();
    let mut index = 0usize;
    let mut meet_nb = false;
    loop {
        if index >= s.len() {
            break;
        }
        let c = s.chars().nth(index).unwrap();
        match c {
            '+' => {
                ops.push_back(Op::Add);
                meet_nb = false;
            }
            '*' => {
                ops.push_back(Op::Mul);
                meet_nb = false;
            }
            '(' => {
                let end_index = get_end_index_of_expression(&s[index..]) + index;
                let result_of_expression =
                    evaluate_expression(&s[index + 1..end_index - 1], same_precedence_level);
                meet_nb = true;
                if !ops.is_empty()
                    && values.len() >= 1
                    && (same_precedence_level || (*ops.back().unwrap() == Op::Add && meet_nb))
                {
                    let left_operand = values.pop_back().unwrap();
                    let operator = ops.pop_back().unwrap();
                    values.push_back(operator.compute(left_operand, result_of_expression));
                } else {
                    values.push_back(result_of_expression);
                }
                index = end_index;
            }
            ')' => {
                meet_nb = true;
            }
            ' ' => {
                if !acc.is_empty() {
                    values.push_back(acc.parse::<isize>().unwrap());
                    acc.clear();
                }
                if !ops.is_empty() && values.len() >= 2 {
                    if same_precedence_level || (*ops.back().unwrap() == Op::Add && meet_nb) {
                        let right_operand = values.pop_back().unwrap();
                        let left_operand = values.pop_back().unwrap();
                        let operator = ops.pop_back().unwrap();
                        values.push_back(operator.compute(left_operand, right_operand));
                    }
                }
            }
            _ => {
                acc.push(c);
                meet_nb = true;
            }
        }
        index += 1;
    }
    if acc.is_empty() && ops.is_empty() {
        return values.pop_back().unwrap();
    }
    if !acc.is_empty() {
        values.push_back(acc.parse::<isize>().unwrap());
    }
    let mut right_operand = values.pop_back().unwrap();
    while !ops.is_empty() {
        let operator = ops.pop_back().unwrap();
        let left_operand = values.pop_back().unwrap();
        right_operand = operator.compute(left_operand, right_operand);
    }
    return right_operand;
}

#[cfg(test)]
mod tests {
    use super::evaluate_expression;
    #[test]
    fn test_evaluate_expression_same_precedence() {
        assert_eq!(evaluate_expression("1 + 2 * 3 + 4 * 5 + 6", true), 71);
        assert_eq!(evaluate_expression("1 + (2 * 3) + (4 * (5 + 6))", true), 51);
        assert_eq!(evaluate_expression("2 * 3 + (4 * 5)", true), 26);
        assert_eq!(
            evaluate_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)", true),
            437
        );
        assert_eq!(
            evaluate_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true),
            12240
        );
        assert_eq!(
            evaluate_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true),
            13632
        );
    }

    #[test]
    fn test_evaluate_expression_add_precedence() {
        assert_eq!(evaluate_expression("1 + 2 * 3 + 4 * 5 + 6", false), 231);
        assert_eq!(
            evaluate_expression("1 + (2 * 3) + (4 * (5 + 6))", false),
            51
        );
        assert_eq!(evaluate_expression("2 * 3 + (4 * 5)", false), 46);
        assert_eq!(
            evaluate_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)", false),
            1445
        );
        assert_eq!(
            evaluate_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false),
            669060
        );
        assert_eq!(
            evaluate_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false),
            23340
        );
    }
}
