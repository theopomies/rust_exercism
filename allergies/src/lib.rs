use enum_iterator::IntoEnumIterator;

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator)]
pub enum Allergen {
    Eggs = 0,
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
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & (1 << *allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::into_enum_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
