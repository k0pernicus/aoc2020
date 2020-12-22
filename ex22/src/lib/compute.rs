use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

fn copy_deque(deque_to_copy: &VecDeque<usize>, limit: usize) -> VecDeque<usize> {
    let mut copy = deque_to_copy.clone();
    let mut diff = deque_to_copy.len() - limit;
    while diff != 0 {
        copy.pop_back();
        diff -= 1;
    }
    copy
}

fn get_score(deque: &mut VecDeque<usize>) -> usize {
    let mut score = 0;
    let mut weight = 1;
    while !deque.is_empty() {
        let card = deque.pop_back().unwrap();
        score += card * weight;
        weight += 1;
    }
    return score;
}

pub fn get_winner_part_1(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
) -> Result<(usize, usize), ()> {
    let mut winner = 1;
    loop {
        if p1.is_empty() {
            winner = 2;
            break;
        }
        if p2.is_empty() {
            winner = 1;
            break;
        }
        let (card_p1, card_p2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        // Compute the winner
        match card_p1.cmp(&card_p2) {
            Ordering::Less => {
                p2.push_back(card_p2);
                p2.push_back(card_p1);
            }
            Ordering::Greater => {
                p1.push_back(card_p1);
                p1.push_back(card_p2);
            }
            Ordering::Equal => {
                println!("Situation may not happened... bad inputs?");
            }
        }
    }

    let mut winner_deque = match winner {
        1 => p1,
        2 => p2,
        _ => {
            println!("Error: cannot have player with id {}", winner);
            return Err(());
        }
    };

    let score = get_score(&mut winner_deque);

    Ok((winner, score))
}

pub fn get_winner_part_2(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
) -> Result<(usize, usize), ()> {
    let mut winner = None;
    let mut sub_games: HashMap<VecDeque<usize>, Vec<VecDeque<usize>>> = HashMap::new();

    loop {
        if p1.is_empty() {
            winner = Some(2);
            break;
        }
        if p2.is_empty() {
            winner = Some(1);
            break;
        }
        // Check if already played
        match sub_games.get(&p1) {
            Some(games) => {
                if games.contains(&p2) {
                    return Ok((1, get_score(&mut p1)));
                }
            }
            None => sub_games
                .entry(p1.clone())
                .or_insert(Vec::new())
                .push(p2.clone()),
        }

        let (card_p1, card_p2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        if card_p1 <= p1.len() && card_p2 <= p2.len() {
            let cpy_p1 = copy_deque(&p1, card_p1);
            let cpy_p2 = copy_deque(&p2, card_p2);
            match get_winner_part_2(cpy_p1, cpy_p2) {
                Ok((subgame_winner, _)) => match subgame_winner {
                    1 => {
                        p1.push_back(card_p1);
                        p1.push_back(card_p2);
                    }
                    2 => {
                        p2.push_back(card_p2);
                        p2.push_back(card_p1);
                    }
                    _ => {
                        println!(
                            "Error: got player {} as a subgame winner...",
                            subgame_winner
                        );
                        return Err(());
                    }
                },
                Err(_) => {
                    println!("Error when playing a subgame...");
                    return Err(());
                }
            }
        } else {
            match card_p1.cmp(&card_p2) {
                Ordering::Less => {
                    p2.push_back(card_p2);
                    p2.push_back(card_p1);
                }
                Ordering::Greater => {
                    p1.push_back(card_p1);
                    p1.push_back(card_p2);
                }
                Ordering::Equal => {
                    println!("Situation may not happened... bad inputs?");
                }
            }
        }
    }

    match winner {
        Some(winner_id) => match winner_id {
            1 => {
                return Ok((1, get_score(&mut p1)));
            }
            2 => {
                return Ok((2, get_score(&mut p2)));
            }
            _ => {
                println!("Error: found winner {}, but expected 1 or 2", winner_id);
                return Err(());
            }
        },
        None => {
            println!("No winner...");
            return Err(());
        }
    }
}
