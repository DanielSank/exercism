use std::collections::HashSet;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

pub struct Allergies {
    allergens: HashSet<Allergen>
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, EnumIter)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergens = HashSet::new();
        let mut powers = HashSet::new();
        let mut this_power = 0;
        while (1 << this_power) <= score {
            if (1 << this_power) & score != 0 {
                powers.insert(this_power);
            }
            this_power += 1;
        }
        for (idx, allergen) in Allergen::iter().enumerate() {
            if powers.contains(&u32::try_from(idx).unwrap()) {
                allergens.insert(allergen);
            }
        }
        Allergies{allergens}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut v = vec![];
        for allergen in self.allergens.iter() {
            v.push(*allergen)
        }
        v.sort();
        v
    }
}

pub fn main() {
    let _allergies = Allergies::new(24);
}
