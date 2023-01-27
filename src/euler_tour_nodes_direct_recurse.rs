pub fn euler_tour_nodes(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<usize> {
    let n = g.len();

    let mut tour = Vec::with_capacity(2 * n - 1);

    fn dfs(
        g: &[Vec<usize>],
        tour: &mut Vec<usize>,
        u: usize,
        p: usize,
    ) {
        tour.push(u);

        for &v in g[u].iter() {
            if v == p {
                continue;
            }

            dfs(g, tour, v, u);

            tour.push(u);
        }
    }

    dfs(g, &mut tour, root, n);

    tour
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];

        assert_eq!(euler_tour_nodes(&g, 0), [0, 1, 3, 1, 0, 2, 0]);
    }
}
