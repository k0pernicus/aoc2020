use super::Food;
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn solve_part_1_and_2(foods: &Vec<Food>) -> (HashMap<String, usize>, BTreeMap<String, String>) {
    let mut allergens: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut all_ingredients = HashMap::<String, usize>::new();
    // Compute the list of all ingredients (and the number of occurences per food)
    for food in foods.iter() {
        for ingredient in food.ingredients.0.iter() {
            *all_ingredients.entry(ingredient.clone()).or_insert(0) += 1;
        }
    }
    // Compute the allergenes
    for food in foods.iter() {
        for allergen in food.allergens.0.iter() {
            for ingredient in food.ingredients.0.iter() {
                *allergens
                    .entry(allergen.clone())
                    .or_insert(HashMap::new())
                    .entry(ingredient.clone())
                    .or_insert(0) += 1;
            }
        }
    }
    let mut found = false;
    let mut ingredients_with_allergenes = HashSet::<String>::new();
    let mut ingredients_per_allergene: BTreeMap<String, String> = BTreeMap::new();
    while !found {
        found = true;
        for (allergene, all_possible_ingredients) in allergens.iter() {
            let max = all_possible_ingredients.values().max().unwrap();
            let mut possible_ingredients: Vec<String> = all_possible_ingredients
                .iter()
                .filter_map(|(ingredient, value)| {
                    if value == max {
                        return Some(ingredient.clone());
                    } else {
                        None
                    }
                })
                .collect();
            if possible_ingredients.len() > 1 {
                possible_ingredients.retain(|x| !ingredients_with_allergenes.contains(x));
            }
            if possible_ingredients.len() == 1 {
                ingredients_with_allergenes.insert(possible_ingredients[0].clone());
                ingredients_per_allergene
                    .insert(allergene.clone(), possible_ingredients[0].clone());
                continue;
            } else if possible_ingredients.len() > 1 {
                // Don't compute twice the same keys
                let mut new_possible_ingredients = HashMap::<String, usize>::new();
                for (ingredient, _) in all_possible_ingredients.clone() {
                    if !possible_ingredients.contains(&ingredient) {
                        new_possible_ingredients.insert(
                            ingredient.clone(),
                            *all_possible_ingredients.get(&ingredient).unwrap(),
                        );
                    }
                }
                found = false;
            }
        }
    }
    // Sort part 1
    all_ingredients.retain(|ingredient, _| !ingredients_with_allergenes.contains(ingredient));
    (all_ingredients, ingredients_per_allergene)
}
