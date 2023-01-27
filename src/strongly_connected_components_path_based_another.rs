pub fn scc(g: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = g.len();

    let mut components = vec![];

    let mut preorder = Vec::with_capacity(n);

    let mut low = Vec::with_capacity(n);

    let mut state = vec![0; n];

    let mut st_dfs: Vec<_> = (0..n as isize).rev().collect();

    while let Some(u) = st_dfs.pop() {
        if u < 0 {
            let u = !u as usize;

            let ord = state[u] - 1;

            if *low.last().unwrap() <= ord {
                continue;
            }

            let mut group = Vec::with_capacity(preorder.len() - ord);

            while preorder.len() > ord {
                let v = preorder.pop().unwrap();

                group.push(v);

                state[v] = n + 1;
            }

            low.pop();

            components.push(group);

            continue;
        }

        let u = u as usize;

        if state[u] == n + 1 {
            continue;
        }

        if state[u] > 0 {
            while *low.last().unwrap() > state[u] {
                low.pop();
            }

            continue;
        }

        preorder.push(u);

        let ord = preorder.len();

        low.push(ord);

        state[u] = ord;

        st_dfs.push(!u as isize);

        for &v in g[u].iter() {
            st_dfs.push(v as isize);
        }
    }

    components.reverse();

    components
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (6, vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)]),
            vec![vec![5], vec![4, 1], vec![2], vec![3, 0]],
        )];

        for ((n, edges), ans) in cases {
            let mut g = vec![vec![]; n];

            for (u, v) in edges {
                g[u].push(v);
            }

            assert_eq!(scc(&g), ans);
        }
    }
}
