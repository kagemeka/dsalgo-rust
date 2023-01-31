//! similar to array_compression_with_argsort.rs
//! but each rank is unique.
//! if same value, sort by original index.

use crate::{
    argsort::argsort,
    permutation_argsort::argsort as perm_argsort,
};

pub fn array_rank<T: Ord>(a: &[T]) -> Vec<usize> { perm_argsort(&argsort(a)) }

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![3, -1, 2, 5];

        assert_eq!(array_rank(&a), [2, 0, 1, 3]);
    }
}
