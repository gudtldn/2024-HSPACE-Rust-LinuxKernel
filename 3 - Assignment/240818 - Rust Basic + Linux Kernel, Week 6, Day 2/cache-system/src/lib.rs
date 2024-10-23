use std::collections::HashMap;

#[allow(dead_code)]
fn check_cache<T, F>(closure: F) -> impl FnMut(T) -> (T, bool)
where
    T: std::ops::Mul + std::hash::Hash + std::cmp::Eq + Copy,
    F: Fn(T) -> T,
{
    let mut cache = HashMap::new();
    move |x| match cache.get(&x) {
        Some(value) => (*value, true),
        None => {
            let result = closure(x);
            cache.insert(x, result);
            (result, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check_cache;

    #[test]
    fn test_compute_and_cache() {
        let mut compute_func = check_cache(|x| x * x);

        assert_eq!(compute_func(4), (16, false)); // Compute: 16
        assert_eq!(compute_func(4), (16, true)); // Use cache: 16
        assert_eq!(compute_func(5), (25, false)); // Compute: 25
    }
}
