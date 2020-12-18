use std::collections::VecDeque;
use std::ops;
use std::process;

fn find_sub_expression<'a>(expr: &'a str) -> &'a str {
    println!("subexpr: {}", expr);
    if expr.len() == 0 {
        return "0";
    }
    let mut parenthesis_deque: VecDeque<char> = VecDeque::new();
    let mut end_index = 0usize;
    for (index, c) in expr.chars().enumerate() {
        match c {
            '(' => parenthesis_deque.push_back('('),
            ')' => {
                if parenthesis_deque.is_empty() {
                    end_index = index;
                    break;
                }
                parenthesis_deque.pop_back();
            }
            _ => {}
        }
    }
    &expr[..end_index]
}

pub fn evaluate_expression(expr: &str) -> i32 {
    let c_expr = expr.clone();
    let c_expr = c_expr.replace("(", " ( ");
    let c_expr = c_expr.replace(")", " ) ");
    let tokens = c_expr.split_ascii_whitespace();
    let mut sum = 0i32;
    let mut c_ops: Option<fn(i32, i32) -> i32> = None;
    for (index, token) in tokens.into_iter().enumerate() {
        println!("Token {}", token);
        match token {
            "+" => c_ops = Some(ops::Add::add),
            "-" => c_ops = Some(ops::Sub::sub),
            "*" => c_ops = Some(ops::Mul::mul),
            "/" => c_ops = Some(ops::Div::div),
            "(" => {
                let subexpr = tokens.clone();
                let found_subexpr = find_sub_expression(&subexpr);
                let evaluate_subexpr = evaluate_expression(found_subexpr);
                sum += evaluate_subexpr;
            }
            ")" => {}
            _ => match c_ops {
                Some(operator) => sum = operator(sum, token.parse::<i32>().unwrap()),
                None => {
                    if index != 0 {
                        println!("Parsing error: expected an error, got None...");
                        process::exit(1);
                    }
                    sum = token.parse::<i32>().unwrap();
                }
            },
        }
    }
    sum
}
