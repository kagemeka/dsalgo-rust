pub fn euler_tour_edges(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<isize> {
    let n = g.len();

    let mut tour = Vec::with_capacity(n << 1);

    fn dfs(
        g: &[Vec<usize>],
        tour: &mut Vec<isize>,
        u: usize,
        p: usize,
    ) {
        tour.push(u as isize);

        for &v in g[u].iter() {
            if v != p {
                dfs(g, tour, v, u);
            }
        }

        tour.push(!u as isize);
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

        assert_eq!(euler_tour_edges(&g, 0), [0, 1, 3, -4, -2, 2, -3, -1]);
    }
}
