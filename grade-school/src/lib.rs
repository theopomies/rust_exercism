use std::collections::BTreeMap;

trait Sorted<T> {
    fn insert_into_sorted(&mut self, element: T);
}

impl<T: Ord> Sorted<T> for Vec<T> {
    fn insert_into_sorted(&mut self, element: T) {
        let pos = self.binary_search(&element).err().unwrap();
        self.insert(pos, element);
    }
}

pub struct School {
    db: BTreeMap<u32, Vec<String>>,
}

impl Default for School {
    fn default() -> Self {
        School {
            db: BTreeMap::new(),
        }
    }
}

impl School {
    pub fn new() -> School {
        Self::default()
    }

    pub fn add(&mut self, grade: u32, name: &str) {
        self.db
            .entry(grade)
            .or_insert(vec![])
            .insert_into_sorted(name.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.db.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.db.get(&grade).or(Some(&vec![])).unwrap().clone()
    }
}
