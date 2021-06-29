#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

use Comparison::*;

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Equal
    } else if contains(first_list, second_list) {
        Superlist
    } else if contains(second_list, first_list) {
        Sublist
    } else {
        Unequal
    }
}

fn contains<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() < second_list.len() {
        false
    } else if first_list.starts_with(second_list) {
        true
    } else {
        contains(&first_list[1..], second_list)
    }
}
