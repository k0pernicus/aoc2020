use std::collections::HashMap;

// get_jolts_difference_per_adapter needs a SORTED adapter vector!
// returns the differences of 1-jolts, 2-jolts, and 3-jolts - in this specific order
pub fn get_jolts_difference_per_adapter(adapters: &Vec<usize>) -> HashMap<usize, usize> {
    let adapters_len = adapters.len();
    let mut jolts_diff = HashMap::new();
    for (index, adapter) in adapters.iter().enumerate() {
        if index == 0 {
            *jolts_diff.entry(*adapter).or_insert(0) += 1usize;
        } else if (index + 1) >= adapters_len {
            *jolts_diff.entry(3).or_insert(0) += 1usize;
            continue;
        }
        let next_adapter = adapters[index + 1];
        *jolts_diff.entry(next_adapter - adapter).or_insert(0) += 1usize;
    }
    jolts_diff
}

pub fn get_solution_to_part_1(adapter_differences: HashMap<usize, usize>) -> Option<usize> {
    if !adapter_differences.contains_key(&1) || !adapter_differences.contains_key(&3) {
        return None;
    }
    Some(*adapter_differences.get(&1).unwrap() * *adapter_differences.get(&3).unwrap())
}

pub fn get_nb_of_simple_paths_backtracking(mut adapters: Vec<usize>) -> usize {
    // Prepend and append the min and max values
    adapters.insert(0, 0);
    let max_elt = adapters.iter().max().unwrap() + 3;
    adapters.push(max_elt);
    // Except the first item, put everything to 0
    let mut paths: Vec<usize> = vec![0; adapters.len()];
    paths[0] += 1;
    let nb_adapters = adapters.len();

    for (index, adapter) in adapters.iter().enumerate() {
        for (next_index, next_adapter) in adapters[index + 1..(index + 4).min(nb_adapters)]
            .iter()
            .enumerate()
        {
            let c_index = index + 1 + next_index;
            if *next_adapter <= (adapter + 3) {
                // println!("added one!");
                paths[c_index] += paths[index];
            }
        }
    }

    // Return the last one
    *paths.last().unwrap()
}
