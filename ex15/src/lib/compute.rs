use std::collections::HashMap;

fn manage_vector(v: &mut Vec<usize>, new_data: usize) {
    let len_vec = v.len();
    if len_vec <= 1 {
        v.push(new_data);
        return;
    }
    v[0] = v[1];
    v[1] = new_data;
}

pub fn compute_nth_turn(init_list: Vec<isize>, total_turns: usize) -> isize {
    let mut info_per_nb: HashMap<isize, Vec<usize>> = HashMap::new();
    for (turn_index, init_nb) in init_list.iter().enumerate() {
        let mut v = Vec::with_capacity(2);
        v.push(turn_index + 1);
        //println!("Turn {} -> {}", turn_index + 1, init_nb);
        info_per_nb.insert(*init_nb, v);
    }
    let mut last_spoken_nb = init_list[init_list.len() - 1];
    for turn in init_list.len() + 1..total_turns + 1 {
        if !info_per_nb.contains_key(&last_spoken_nb) {
            last_spoken_nb = 0;
        } else {
            let last_indexes = info_per_nb.get(&last_spoken_nb).unwrap();
            let nb_indexes = last_indexes.len();
            if nb_indexes == 1 {
                last_spoken_nb = 0;
            } else {
                last_spoken_nb =
                    (last_indexes[nb_indexes - 1] - last_indexes[nb_indexes - 2]) as isize;
            }
        }
        let vector = info_per_nb
            .entry(last_spoken_nb)
            .or_insert(Vec::with_capacity(2));
        manage_vector(vector, turn);
    }
    last_spoken_nb
}
