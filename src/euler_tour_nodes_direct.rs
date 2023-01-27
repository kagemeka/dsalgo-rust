pub fn euler_tour_nodes(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<usize> {
    let n = g.len();

    let size = 2 * n - 1;

    let mut tour = vec![n; size];

    let mut st = vec![root as isize];

    let mut parent = vec![n; n];

    for i in 0..size {
        let u = st.pop().unwrap();

        if u < 0 {
            tour[i] = !u as usize;

            continue;
        }

        let u = u as usize;

        tour[i] = u;

        for &v in g[u].iter().rev() {
            if v == parent[u] {
                continue;
            }

            parent[v] = u;

            st.push(!u as isize);

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

        assert_eq!(euler_tour_nodes(&g, 0), [0, 1, 3, 1, 0, 2, 0]);
    }
}
