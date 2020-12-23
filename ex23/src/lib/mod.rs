pub mod parser;

pub enum ComputeLiteralError {
    NoCup,
}

pub type ComputeError<T> = Result<T, ComputeLiteralError>;
#[derive(Debug)]
pub struct Cups(Vec<u32>);

macro_rules! pretty_print_cups {
    ($cups: expr) => {
        $cups
            .iter()
            .map(|item| std::char::from_digit(**item, 10).unwrap())
            .collect::<String>()
    };
}

impl Cups {
    fn new(v: Vec<u32>) -> Cups {
        Cups(v)
    }

    pub fn play_round(&mut self, index: usize) -> ComputeError<()> {
        println!("Playing round {}", index);
        if index > 0 {
            // TODO: Fix performance issues in moving only the references
            let item = self.0.remove(0);
            self.0.push(item);
        }
        // Move to the current cup in a cycle iterator
        let mut cups_iter = self.0.iter();
        // Check if the is a cup...
        let current_cup = cups_iter.by_ref().take(1).next();
        if current_cup.is_none() {
            return Err(ComputeLiteralError::NoCup);
        }
        // Get the current cup
        let current_cup = current_cup.unwrap();
        // Skip the next three cups
        let next_three_cups = cups_iter.by_ref().take(3).collect::<Vec<&u32>>();
        // Get the max cup in the remaining cups
        let remaining_cups = cups_iter.clone().collect::<Vec<&u32>>();
        let max_cup = remaining_cups.iter().max().unwrap();
        // Compute the next cup
        let mut destination_cup: u32 = if *current_cup <= 0 {
            *(*max_cup)
        } else {
            current_cup - 1
        };
        // TODO: Do the computation
        while !remaining_cups.contains(&&destination_cup) {
            if destination_cup == 0 {
                destination_cup = *(*max_cup);
                continue;
            }
            destination_cup -= 1;
        }
        let destination_cup_index = remaining_cups
            .iter()
            .position(|&&x| x == destination_cup)
            .unwrap_or(0);
        // Now, concatenate... self[..index] + remaining_cups[..destination] + next_three_cups + remaining_cups[destination..]
        self.0 = self.0[..1]
            .iter()
            .chain(
                remaining_cups[..destination_cup_index + 1]
                    .to_vec()
                    .into_iter(),
            )
            .chain(next_three_cups.into_iter())
            .chain(
                remaining_cups[destination_cup_index + 1..]
                    .to_vec()
                    .into_iter(),
            )
            .map(|&x| x)
            .collect::<Vec<u32>>();
        Ok(())
    }

    pub fn collect_from(&self, cup_id_to_found: u32) -> Option<Vec<&u32>> {
        match self.0.iter().position(|&cup_id| cup_id == cup_id_to_found) {
            Some(position) => Some(
                self.0[position..]
                    .iter()
                    .chain(self.0[..position].iter())
                    .collect::<Vec<&u32>>(),
            ),
            None => None,
        }
    }
}
