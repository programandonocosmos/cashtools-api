pub fn first_or<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    match a {
        Some(v) => Some(v),
        None => b,
    }
}
