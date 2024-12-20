#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let superlist = _second_list.is_empty()
        || _first_list
            .windows(_second_list.len())
            .any(|window| window == _second_list);
    let sublist = _first_list.is_empty()
        || _second_list
            .windows(_first_list.len())
            .any(|window| window == _first_list);

    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}
