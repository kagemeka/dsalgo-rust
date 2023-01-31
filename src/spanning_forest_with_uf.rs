use crate::{
    union_find_low_memory_with_trait::UnionFind,
    union_find_traits::*,
};

pub fn spanning_forest(
    v_size: usize,
    edges: &[(usize, usize)],
) -> Vec<usize> {
    let mut ids = vec![];

    let mut uf = UnionFind::new(v_size);

    for (i, &(u, v)) in edges.iter().enumerate() {
        if uf.same(u, v) {
            continue;
        }

        uf.unite(u, v);

        ids.push(i);
    }

    ids
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 6;

        let edges = vec![(0, 1), (1, 2), (3, 5), (2, 0), (4, 3)];

        assert_eq!(spanning_forest(n, &edges), [0, 1, 2, 4]);
    }
}
