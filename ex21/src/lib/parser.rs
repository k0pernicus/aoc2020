use super::{Allergens, Food, Ingredients};
use std::str::FromStr;

pub fn parse_input(v: Vec<String>) -> Result<Vec<Food>, ()> {
    let mut all_food = Vec::new();
    for s in v.iter() {
        let mut r_s = s.clone();
        r_s = r_s.replace("(", "");
        r_s = r_s.replace(")", "");
        let separator_food: Vec<&str> = r_s.split("contains").collect();
        if separator_food.len() != 2 {
            println!(
                "Error: cannot parse the food - expected split of two elements, got {}",
                separator_food.len()
            );
            return Err(());
        }
        let (raw_ingredients, raw_allergens) = (separator_food[0], separator_food[1]);
        all_food.push(Food::new(
            Ingredients::from_str(raw_ingredients).unwrap(),
            Allergens::from_str(raw_allergens).unwrap(),
        ))
    }
    Ok(all_food)
}
