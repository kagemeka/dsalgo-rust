pub fn euler_tour_edges(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<isize> {
    let n = g.len();

    let mut tour = vec![0; n << 1];

    let mut st = vec![root as isize];

    let mut parent = vec![n; n];

    for i in 0..n << 1 {
        let u = st.pop().unwrap();

        if u < 0 {
            tour[i] = u;

            continue;
        }

        tour[i] = u;

        st.push(!u);

        let u = u as usize;

        for &v in g[u].iter().rev() {
            if v == parent[u] {
                continue;
            }

            parent[v] = u;

            st.push(v as isize);
        }
    }

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
