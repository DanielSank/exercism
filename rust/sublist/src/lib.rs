#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() == 0 {return true;}
    if a.len() > b.len() {return false;}

    for b_idx in 0..(b.len() - 1) {
        let b_remaining_length = b.len() - b_idx;
        if b_remaining_length < a.len() {return false;}
        if a[0..] == b[b_idx..b_idx + a.len()] {return true;}
    }
    return false;
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if is_sublist(a, b) && is_sublist(b, a) {
        return Comparison::Equal;
    }
    if is_sublist(a, b) {
        return Comparison::Sublist;
    }
    if is_sublist(b, a) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
