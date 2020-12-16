use super::{ClassRanges, Puzzle, Ticket};
use std::collections::HashSet;

pub fn get_wrong_values_in_nearby_tickets(puzzle: &Puzzle) -> Vec<(usize, isize)> {
    let mut wrong_values = Vec::new();

    // Build a hashset to go faster...
    let mut hash = HashSet::<isize>::new();
    for class_ranges in puzzle.header.iter() {
        for raw_range in class_ranges.ranges.iter() {
            let range = raw_range.0;
            for i in range.0..(range.1 + 1) {
                hash.insert(i);
            }
        }
    }

    for (nearby_ticket_index, nearby_ticket) in puzzle.nearby_tickets.iter().enumerate() {
        for nb in nearby_ticket.0.iter() {
            if hash.contains(nb) {
                continue;
            }
            wrong_values.push((nearby_ticket_index, *nb));
            break;
        }
    }

    wrong_values
}

fn get_header_order_per_ticket<'a>(
    header: &'a Vec<ClassRanges>,
    ticket: &Ticket,
) -> Vec<HashSet<&'a str>> {
    let mut v: Vec<HashSet<&'a str>> = Vec::with_capacity(ticket.0.len());
    for nb in ticket.0.iter() {
        let mut hash: HashSet<&'a str> = HashSet::new();
        for class in header.iter() {
            let low_range = &class.ranges[0];
            let high_range = &class.ranges[1];
            if low_range.contains(*nb) || high_range.contains(*nb) {
                hash.insert(class.name.as_str());
            }
        }
        v.push(hash)
    }
    v
}

pub fn get_header_order<'a>(puzzle: &'a Puzzle) -> Vec<&'a str> {
    let mut initial_set = get_header_order_per_ticket(&puzzle.header, &puzzle.my_ticket);
    println!("Initial set: {:?}", initial_set);
    for nearby_ticket in puzzle.nearby_tickets.iter() {
        let new_set = get_header_order_per_ticket(&puzzle.header, &nearby_ticket);
        for (index, set) in new_set.into_iter().enumerate() {
            let remaining_class_names = initial_set[index]
                .intersection(&set)
                .into_iter()
                .collect::<HashSet<_>>();
            println!("> Remaining class names: {:?}", remaining_class_names);
            let mut copy_initial_set = initial_set[index].clone();
            for elt in initial_set[index].iter() {
                if remaining_class_names.contains(elt) {
                    continue;
                }
                let _ = copy_initial_set.remove(elt);
            }
            println!(">> After removal: {:?}", copy_initial_set);
            initial_set[index] = copy_initial_set;
        }
    }
    println!("BEFORE: {:?}", initial_set);
    loop {
        let mut unique_values = HashSet::new();
        // First step: get the unique values
        for values in initial_set.iter() {
            if values.len() == 1 {
                unique_values.insert(values.into_iter().next().unwrap());
            }
        }
        if unique_values.len() == initial_set.len() {
            break;
        }
        let mut copy_initial_set = initial_set.clone();
        // Second step: filter
        for (index, values) in initial_set.iter().enumerate() {
            if values.len() == 1 {
                continue;
            }
            for unique_value in unique_values.iter() {
                if values.contains(*unique_value) {
                    copy_initial_set[index].remove(*unique_value);
                }
            }
        }
        initial_set = copy_initial_set;
    }
    println!("AFTER: {:?}", initial_set);
    initial_set
        .into_iter()
        .map(|x| x.into_iter().next().unwrap())
        .collect()
}
