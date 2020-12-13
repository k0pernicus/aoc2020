use super::{BusID, BusIDs, Timestamp};

pub fn parse_raw_input(raw_input: &Vec<String>) -> (Timestamp, BusIDs) {
    let timestamp = raw_input[0].parse::<usize>().unwrap();
    let bus_ids = raw_input[1]
        .split(",")
        .filter(|id| *id != "x")
        .map(|id| BusID(id.parse::<usize>().unwrap()))
        .collect::<Vec<BusID>>();

    return (Timestamp(timestamp), bus_ids);
}

pub fn parse_with_internal_departures(raw_input: &Vec<String>) -> Vec<(BusID, Timestamp)> {
    raw_input[1]
        .split(",")
        .enumerate()
        .map(|(index, id)| {
            if id == "x" {
                return None;
            }
            return Some((id, index));
        })
        .filter(|elt| elt.is_some())
        .map(|elt| elt.unwrap())
        .map(|elt| (BusID(elt.0.parse::<usize>().unwrap()), Timestamp(elt.1)))
        .collect()
}
