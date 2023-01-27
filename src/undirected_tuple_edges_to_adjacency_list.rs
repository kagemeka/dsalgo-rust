pub fn to_adjlist(
    v_size: usize,
    edges: Vec<(usize, usize)>,
) -> Vec<Vec<usize>> {
    let mut g = vec![vec![]; v_size];

    for (u, v) in edges.into_iter() {
        g[u].push(v);

        g[v].push(u);
    }

    g
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 4;

        let edges = vec![(0, 2), (2, 1), (3, 0), (2, 3)];

        assert_eq!(
            to_adjlist(n, edges),
            vec![vec![2, 3], vec![2], vec![0, 1, 3], vec![0, 2]]
        );
    }
}
