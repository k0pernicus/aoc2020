const COMMON_DIVIDER_VALUE: usize = 20201227;

pub fn compute_loop_size(subject_number: usize, value_to_find: usize) -> Option<usize> {
    let mut current_value: usize = 1;
    let mut index = 0;
    loop {
        index += 1;
        current_value *= subject_number;
        let multiplicator = current_value / COMMON_DIVIDER_VALUE;
        current_value = current_value - (multiplicator * COMMON_DIVIDER_VALUE);
        if current_value == value_to_find {
            break;
        }
    }
    Some(index)
}

pub fn compute_encryption_key(subject_number: usize, loop_size: usize) -> usize {
    let mut current_value: usize = 1;
    for _ in 0..loop_size {
        current_value *= subject_number;
        let multiplicator = current_value / COMMON_DIVIDER_VALUE;
        current_value = current_value - (multiplicator * COMMON_DIVIDER_VALUE);
    }
    current_value
}
