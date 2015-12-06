#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Clone, Copy, PartialEq, Debug, IterVariants(Vars))]
    pub enum Allergen {
        Eggs = 1,
        Peanuts = 2,
        Shellfish = 4,
        Strawberries = 8,
        Tomatoes = 16,
        Chocolate = 32,
        Pollen = 64,
        Cats = 128,
    }
}

pub struct Allergies(pub u32);

impl Allergies {

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let a = *allergen as u32;
        self.0 & a != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter_variants()
                 .filter(|a| self.is_allergic_to(&a)).collect::<Vec<Allergen>>()
    }

}
