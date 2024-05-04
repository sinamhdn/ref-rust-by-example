//! one use of patters is catch all pattern in match expression because match must be exhaustive
match x {
    None => None,
    Some(i) => Some(i + 1),
    _ => None,
}
