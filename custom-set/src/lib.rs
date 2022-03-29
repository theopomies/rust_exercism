#[derive(Debug)]
pub struct CustomSet<T>(Vec<T>);

impl<T> CustomSet<T>
where
    T: Clone + Ord,
{
    pub fn new(input: &[T]) -> Self {
        Self(input.into())
    }

    pub fn contains(&self, element: &T) -> bool {
        self.0.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.0.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.iter().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.iter().all(|element| !other.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|element| other.contains(element))
                .cloned()
                .collect(),
        )
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|element| !other.contains(element))
                .cloned()
                .collect(),
        )
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|element| !other.contains(element))
                .chain(other.0.iter())
                .cloned()
                .collect(),
        )
    }
}

impl<T> PartialEq for CustomSet<T>
where
    T: Ord + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|element| other.contains(element))
            && other.0.iter().all(|element| self.contains(element))
    }
}
