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

pub fn Allergies(mask:u32) -> Allergies_{
    Allergies_::new(mask)
}

pub struct Allergies_ {
    mask: u32,
}

impl Allergies_ {
    pub fn new(mask: u32) -> Allergies_ {
        Allergies_ {
            mask: mask,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let a = *allergen as u32;
        self.mask & a != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter_variants()
                 .filter(|a| { let a_ = *a as u32; a_ & self.mask != 0} ).collect::<Vec<Allergen>>()
    }

}
