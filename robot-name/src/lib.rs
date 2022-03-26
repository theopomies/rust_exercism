#![allow(clippy::new_without_default)]

use rand::prelude::*;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
    static NAME_SET: RefCell<HashSet<String>> = RefCell::new(HashSet::new())
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Self::generate_unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        NAME_SET.with(|set| {
            set.borrow_mut().remove(self.name());
            self.name = Self::generate_unique_name()
        })
    }

    fn generate_unique_name() -> String {
        NAME_SET.with(|set| {
            let mut name = Self::generate_name();
            while !set.borrow_mut().insert(name.clone()) {
                name = Self::generate_name();
            }
            name
        })
    }

    fn generate_name() -> String {
        let mut rng = thread_rng();
        let first_letter = rng.gen_range('A'..='Z');
        let second_letter = rng.gen_range('A'..='Z');
        let number = rng.gen_range(0..=999u16);

        format!("{first_letter}{second_letter}{number:03}")
    }
}
