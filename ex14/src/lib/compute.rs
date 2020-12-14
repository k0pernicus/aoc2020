use bit_array::BitArray;
use typenum::U36;

pub fn get_bit_array(value: usize) -> BitArray<u32, U36> {
    let mut bit_array = BitArray::new();
    let binary_value = format!("{:b}", value).chars().rev().collect::<String>();
    for (index, bin) in binary_value.chars().enumerate() {
        bit_array.set(
            //index + instruction.memory_address,
            36 - (index + 1),
            match bin {
                '1' => true,
                _ => false,
            },
        );
    }
    bit_array
}

pub fn to_digit(bit_array: &BitArray<u32, U36>) -> isize {
    let bit_str = bit_array
        .iter()
        .map(|b| {
            if !b {
                return '0';
            }
            return '1';
        })
        .collect::<String>();
    isize::from_str_radix(bit_str.as_str(), 2).unwrap()
}
