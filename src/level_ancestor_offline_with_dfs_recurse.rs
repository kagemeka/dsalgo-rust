pub fn level_ancestor(
    g: &[Vec<usize>],
    root: usize,
    queries: &[(usize, usize)],
) -> Vec<Option<usize>> {
    let n = g.len();

    let mut st = Vec::new();

    let mut res = vec![None; queries.len()];

    let mut q = vec![vec![]; n];

    for (i, &(u, k)) in queries.iter().enumerate() {
        q[u].push((i, k));
    }

    fn dfs(
        g: &[Vec<usize>],
        q: &[Vec<(usize, usize)>],
        res: &mut [Option<usize>],
        st: &mut Vec<usize>,
        u: usize,
        p: usize,
    ) {
        st.push(u);

        for &(i, k) in q[u].iter() {
            if st.len() <= k {
                continue;
            }

            res[i] = Some(st[st.len() - 1 - k]);
        }

        for &v in g[u].iter() {
            if v == p {
                continue;
            }

            dfs(g, q, res, st, v, u);
        }

        st.pop();
    }

    dfs(g, &q, &mut res, &mut st, root, n);

    res
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_atcoder_abc267_f() {
        let cases = vec![(
            5,
            vec![(1, 2), (2, 3), (3, 4), (3, 5)],
            vec![(2, 2), (5, 3), (3, 3)],
            vec![4, 1, -1],
        )];

        use crate::tree_diameter_path_unweighted::diameter_path;

        for (n, edges, mut queries, ans) in cases {
            let mut g = vec![vec![]; n];

            for (mut u, mut v) in edges {
                u -= 1;

                v -= 1;

                g[u].push(v);

                g[v].push(u);
            }

            for q in queries.iter_mut() {
                q.0 -= 1;
            }

            let path = diameter_path(&g);

            let u = path[0];

            let v = *path.last().unwrap();

            let res0 = level_ancestor(&g, u, &queries);

            let res1 = level_ancestor(&g, v, &queries);

            for ((u, v), ans) in
                res0.into_iter().zip(res1.into_iter()).zip(ans.into_iter())
            {
                let res = if let Some(u) = u {
                    u as isize + 1
                } else if let Some(v) = v {
                    v as isize + 1
                } else {
                    -1
                };

                assert_eq!(res, ans);
            }
        }
    }
}
