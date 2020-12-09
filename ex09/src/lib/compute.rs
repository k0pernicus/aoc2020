use std::cmp;
use std::collections::{HashSet, VecDeque};

fn get_combinations_of_preamble(
    data: &Vec<u32>,
    nb_of_preambles: usize,
    index: usize,
) -> HashSet<u32> {
    let mut hashset: HashSet<u32> = HashSet::new();
    let preamble = data[index..index + nb_of_preambles].iter().as_slice();
    for (index, x) in preamble.iter().enumerate() {
        for y in preamble[index + 1..].iter() {
            hashset.insert(x + y);
        }
    }
    hashset
}

pub fn get_first_non_sum_nb(data: &Vec<u32>, nb_of_preambles: usize) -> Option<(u32, usize)> {
    for (index, c_data) in data[nb_of_preambles..].iter().enumerate() {
        let combinations = get_combinations_of_preamble(data, nb_of_preambles, index);
        if !combinations.contains(c_data) {
            return Some((*c_data, index + nb_of_preambles));
        }
    }
    None
}

// get_invalid_range returns the indexes of the data range that sums the nb_to_find parameter
// Brute force version!
#[allow(dead_code)]
pub fn get_invalid_range_bruteforce(data: &[u32], nb_to_find: u32) -> Option<(usize, usize)> {
    let rev_data: Vec<&u32> = data.iter().rev().collect();
    let mut nb_operations = 0;
    for (index, c_value) in rev_data.clone().into_iter().enumerate() {
        let mut sum = *c_value;
        let mut iterator_index = index;
        nb_operations += 1;
        loop {
            nb_operations += 1;
            iterator_index += 1;
            sum += rev_data[iterator_index];
            match sum.cmp(&nb_to_find) {
                cmp::Ordering::Less => continue,
                cmp::Ordering::Equal => {
                    println!("Performed {} operations", nb_operations);
                    let len_data = data.len();
                    return Some((len_data - iterator_index - 1, len_data - index - 1));
                }
                cmp::Ordering::Greater => break,
            }
        }
    }
    None
}

// get_invalid_range_memoization returns the indexes of the data range that sums the nb_to_find parameter
// Optimized version (x4 less computations)!
pub fn get_invalid_range_memoization(data: &[u32], nb_to_find: u32) -> Option<(usize, usize)> {
    let rev_data: Vec<&u32> = data.iter().rev().collect();
    let mut nb_operations = 0;
    let mut queue = VecDeque::new();
    let mut sum = 0;
    let mut iterator_index = 0;
    for (index, c_value) in rev_data.clone().into_iter().enumerate() {
        nb_operations += 1;
        sum += c_value;
        queue.push_back(c_value);
        match sum.cmp(&nb_to_find) {
            cmp::Ordering::Less => {
                continue;
            }
            cmp::Ordering::Equal => {
                println!("Performed {} operations", nb_operations);
                let len_data = data.len();
                return Some((len_data - index - 1, len_data - iterator_index - 1));
            }
            cmp::Ordering::Greater => {
                sum -= queue.pop_front().unwrap();
                iterator_index += 1;
                // Remove the latest one and go on...
            }
        }
    }
    None
}
