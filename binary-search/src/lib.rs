#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::cmp::Ordering::{Equal, Greater, Less};

#[must_use]
pub fn find<T, U>(collection: U, key: T) -> Option<usize>
where
    T: Ord,
    U: AsRef<[T]>,
{
    let collection = collection.as_ref();
    let mid = collection.len() / 2;
    match key.cmp(collection.get(mid)?) {
        Equal => Some(mid),
        Less => find(&collection[..mid], key),
        Greater => find(&collection[mid + 1..], key).map(|i| i + mid + 1),
    }
}
