use lazy_static;
use regex::Regex;
use std::collections::VecDeque;

trait ParseInput {
    fn is_player(&self) -> bool;
}

impl ParseInput for String {
    fn is_player(&self) -> bool {
        self.starts_with("Player")
    }
}

fn parse_player(s: &str) -> Option<usize> {
    lazy_static! {
        static ref PLAYER_RE: Regex = Regex::new(r"Player (\d+):").unwrap();
    }
    match PLAYER_RE.captures(s) {
        Some(captures) => match captures.get(1).map_or("", |u| u.as_str()).parse::<usize>() {
            Ok(u) => return Some(u),
            Err(err) => {
                println!("Error: cannot parse Player UUID: {}", err);
                return None;
            }
        },
        None => None,
    }
}

pub fn get_decks(v: Vec<String>) -> Result<(VecDeque<usize>, VecDeque<usize>), ()> {
    let mut c_player = 0;
    let mut deque_p1: VecDeque<usize> = VecDeque::new();
    let mut deque_p2: VecDeque<usize> = VecDeque::new();
    for s in v.iter() {
        if s.is_empty() {
            continue;
        }
        if s.is_player() {
            match parse_player(s.as_str()) {
                Some(player_uuid) => c_player = player_uuid,
                None => {
                    return Err(());
                }
            }
            continue;
        }
        if c_player > 2 {
            println!("oops, it seems that more than two players are currently playing the game...");
            return Err(());
        }
        if c_player == 1 {
            match s.parse::<usize>() {
                Ok(card_id) => deque_p1.push_back(card_id),
                Err(_) => {
                    println!("Cannot parse card id");
                    return Err(());
                }
            }
            continue;
        }
        deque_p2.push_back(s.parse::<usize>().unwrap());
    }
    Ok((deque_p1, deque_p2))
}
