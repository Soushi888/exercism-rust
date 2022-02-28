#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    } else if _first_list.len() < _second_list.len() && is_sublist(_first_list, _second_list) {
        return Comparison::Sublist;
    } else if _second_list.len() < _first_list.len() && is_sublist(_second_list, _first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(pattern: &[T], search_universe: &[T]) -> bool {
    pattern.is_empty()
        || search_universe
        .windows(pattern.len())
        .any(|w| w == pattern)
}
