use crate::{
    union_find_low_memory_with_trait::UnionFind,
    union_find_traits::*,
};

pub fn has_cycle(
    v_size: usize,
    edges: &[(usize, usize)],
) -> bool {
    let mut uf = UnionFind::new(v_size);

    for &(u, v) in edges.iter() {
        if uf.same(u, v) {
            return true;
        }

        uf.unite(u, v);
    }

    false
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((4, vec![(1, 0), (2, 0)]), false),
            ((4, vec![(1, 0), (2, 0), (2, 1)]), true),
        ];

        for ((n, edges), ans) in cases {
            assert_eq!(has_cycle(n, &edges), ans);
        }
    }
}
