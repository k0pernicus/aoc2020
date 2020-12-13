use super::{BusID, BusIDs, Timestamp};

fn find_divisor(timestamp: &Timestamp, bus_id: &BusID) -> usize {
    if timestamp.0 % bus_id.0 == 0 {
        return timestamp.0 / bus_id.0;
    }
    (timestamp.0 / bus_id.0) + 1
}

fn find_next_timestamp(timestamp: &Timestamp, bus_id: &BusID) -> Timestamp {
    let common_divisor = find_divisor(timestamp, bus_id);
    Timestamp(bus_id.0 * common_divisor)
}

pub fn find_earliest_bus(timestamp: &Timestamp, bus_ids: BusIDs) -> (BusID, Timestamp) {
    let (mut min_bus_id, mut min_timestamp): (Option<BusID>, usize) = (None, 0);
    for bus_id in bus_ids.into_iter() {
        let c_timestamp = find_next_timestamp(&timestamp, &bus_id).0;
        if min_bus_id.is_none() || c_timestamp < min_timestamp {
            min_bus_id = Some(bus_id);
            min_timestamp = c_timestamp;
        }
    }
    (min_bus_id.unwrap(), Timestamp(min_timestamp))
}

// Thanks to rosetta code here: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// Thanks to rosetta code here: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn mod_inv(x: isize, n: isize) -> Option<isize> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

// Thanks to the Chinese Remainder Theorem: https://crypto.stanford.edu/pbc/notes/numbertheory/crt.html
//
// Based on the advent of code test input (7,13,x,x,59,x,31,19), we must solve this:
// t == 0 (mod 7)
// t+1 == 0 (mod 13)
// t+4 == 0 (mod 59)
// t+6 == 0 (mod 31)
// t+7 == 0 (mod 19)
//
// Now, what is "t"?! -> CRT
pub fn find_earliest_timestamp_for_all_buses(bus_ids: Vec<(BusID, Timestamp)>) -> Timestamp {
    // The BusID type corresponds here to the modulo in the formula
    // The timestamp correspond to the x in "t + x" formula
    let first_timestamp = Timestamp(bus_ids[0].0 .0);
    let first_departures: Vec<Timestamp> = bus_ids
        .iter()
        .map(|(_, offset)| first_timestamp.clone() + offset.clone())
        .collect(); // Corresponds to "x"
    println!("First departures: {:?}", first_departures);
    let mut x = 0isize;
    let moduli_product = bus_ids.iter().fold(1isize, |product, modulo| {
        return product * modulo.0 .0 as isize;
    });
    for (index, first_departure) in first_departures.iter().enumerate() {
        let bi = moduli_product / bus_ids[index].0 .0 as isize;
        let res_modulo = bus_ids[index].0 .0 as isize
            - (first_departure.clone() - first_timestamp.clone()).0 as isize;
        x += res_modulo as isize
            * mod_inv(bi as isize, bus_ids[index].0 .0 as isize).unwrap()
            * bi as isize;
    }

    Timestamp((x % moduli_product) as usize)
}
