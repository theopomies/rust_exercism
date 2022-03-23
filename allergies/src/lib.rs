use enum_iterator::IntoEnumIterator;

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Copy, Clone, IntoEnumIterator)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::into_enum_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}

