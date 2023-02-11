pub fn first_or<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    match a {
        Some(v) => Some(v),
        None => b,
    }
}

pub fn invert<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}