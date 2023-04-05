pub fn first_or<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    match a {
        Some(v) => Some(v),
        None => b,
    }
}

pub fn opt_result_of_result_opt<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

pub fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
