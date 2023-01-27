//! manage minimum weigted edges.

pub fn dense_graph_from_edges<T: Clone + Ord>(
    inf: T,
    n: usize,
    edges: Vec<(usize, usize, T)>,
) -> Vec<Vec<T>> {
    let mut g = vec![vec![inf; n]; n];

    for (u, v, w) in edges.into_iter() {
        let x = &mut g[u][v];

        if &w < x {
            *x = w;
        }
    }

    g
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let inf = 1 << 30;

        let cases = vec![(
            vec![
                (0, 0, 0),
                (0, 1, 1),
                (1, 1, 0),
                (1, 2, 2),
                (1, 2, 1),
                (2, 1, 3),
                (2, 2, 0),
            ],
            vec![vec![0, 1, inf], vec![inf, 0, 1], vec![inf, 3, 0]],
        )];

        for (edges, ans) in cases {
            assert_eq!(dense_graph_from_edges(inf, ans.len(), edges), ans);
        }
    }
}
