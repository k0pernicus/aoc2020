pub mod compute;
pub mod parser;

use std::collections::HashSet;
use std::default::Default;
use std::str::FromStr;

#[derive(Debug)]
pub struct Ingredients(HashSet<String>);

impl FromStr for Ingredients {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ingredients(
            s.split_ascii_whitespace()
                .map(|s| String::from(s.trim()))
                .collect::<HashSet<String>>(),
        ))
    }
}

impl Default for Ingredients {
    fn default() -> Self {
        Ingredients(HashSet::new())
    }
}

impl Ingredients {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub struct Allergens(HashSet<String>);

impl FromStr for Allergens {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Allergens(
            s.split(",")
                .map(|s| String::from(s.trim()))
                .collect::<HashSet<String>>(),
        ))
    }
}

impl Default for Allergens {
    fn default() -> Self {
        Allergens(HashSet::new())
    }
}

#[derive(Debug)]
pub struct Food {
    ingredients: Ingredients,
    allergens: Allergens,
}

impl Food {
    pub fn new(ingredients: Ingredients, allergens: Allergens) -> Self {
        Food {
            ingredients,
            allergens,
        }
    }
}
