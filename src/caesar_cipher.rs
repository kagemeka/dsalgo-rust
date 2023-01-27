use std::hash::Hash;

/// caesart cipher

pub fn encrypt<T: Eq + Hash + Clone>(
    table: &[T],
    a: &[T],
    k: usize,
) -> Vec<T> {
    let mut idx = std::collections::HashMap::new();

    for (i, x) in table.iter().enumerate() {
        idx.insert(x.clone(), i);
    }

    let n = table.len();

    assert_eq!(idx.len(), n);

    a.iter().map(|x| table[(idx[x] + k) % n].clone()).collect()
}

pub fn decrypt<T: Eq + Hash + Clone>(
    table: &[T],
    a: &[T],
    k: usize,
) -> Vec<T> {
    let mut idx = std::collections::HashMap::new();

    for (i, x) in table.iter().enumerate() {
        idx.insert(x.clone(), i);
    }

    let n = table.len();

    assert_eq!(idx.len(), n);

    a.iter().map(|x| table[(idx[x] + n - k) % n].clone()).collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::const_ascii_characters::ASCII_LOWERCASES;

        let s = "acb".as_bytes();

        assert_eq!(
            encrypt(ASCII_LOWERCASES.as_bytes(), s, 24),
            "yaz".as_bytes()
        );
    }
}
