pub fn get_two_entries_that_sum(entries: Vec<i32>, sum_to_have: i32) -> Option<(i32, i32)> {
	if entries.len() == 0 {
		return None;
	}
	for (index, entry_1) in entries.iter().enumerate() {
		let entries_to_check: &[i32] = &entries[index+1..];
		for entry_2 in entries_to_check {
			if entry_1 + entry_2 == sum_to_have {
				return Some((*entry_1, *entry_2));
			}
		}
	}
	return None;
}

pub fn get_three_entries_that_sum(entries: Vec<i32>, sum_to_have: i32) -> Option<(i32, i32, i32)> {
	if entries.len() == 0 {
		return None;
	}
	for (index, entry_1) in entries.iter().enumerate() {
		let entries_to_check: &[i32] = &entries[index+1..];
		for entry_2 in entries_to_check {
			let snd_entries_to_check: &[i32] = &entries[index+1+1..];
			for entry_3 in snd_entries_to_check {
				if entry_1 + entry_2 + entry_3 == sum_to_have {
					return Some((*entry_1, *entry_2, *entry_3));
				}
			}
		}
	}
	return None;
}
