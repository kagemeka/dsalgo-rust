//! Inversion (Descrete Math)

use crate::{
    array_compression_unique_binary_search::ArrayCompression,
    fenwick_tree_usize_add_1_indexed::Fenwick,
};

pub fn inversion_number<T: Ord + Clone>(a: &[T]) -> usize {
    let a = ArrayCompression::once(a);

    let mut fw = Fenwick::new(a.len());

    let mut cnt = 0;

    for x in a.into_iter().rev() {
        cnt += fw.get(x);

        fw.add(x, 1);
    }

    cnt as usize
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = vec![0, -1, 3, 5, 2];

        assert_eq!(inversion_number(&a), 3);
    }
}
