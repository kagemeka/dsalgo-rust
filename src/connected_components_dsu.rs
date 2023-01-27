use crate::union_find_low_memory::UnionFind;

pub fn connected_components(
    v_size: usize,
    edges: &[(usize, usize)],
) -> Vec<usize> {
    let mut uf = UnionFind::new(v_size);

    for &(u, v) in edges {
        uf.unite(u, v);
    }

    uf.labels()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 3;

        let edges = vec![(0, 1)];

        assert_eq!(connected_components(n, &edges), [0, 0, 1]);
    }
}
