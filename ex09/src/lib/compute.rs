use std::cmp;
use std::collections::HashSet;

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
pub fn get_invalid_range(data: &[u32], nb_to_find: u32) -> Option<(usize, usize)> {
    let rev_data: Vec<&u32> = data.iter().rev().collect();
    for (index, c_value) in rev_data.clone().into_iter().enumerate() {
        let mut sum = *c_value;
        let mut iterator_index = index;
        loop {
            iterator_index += 1;
            sum += rev_data[iterator_index];
            match sum.cmp(&nb_to_find) {
                cmp::Ordering::Less => continue,
                cmp::Ordering::Equal => {
                    let len_data = data.len();
                    return Some((len_data - iterator_index - 1, len_data - index - 1));
                }
                cmp::Ordering::Greater => break,
            }
        }
    }
    None
}
