use crate::strongly_connected_components_topological_sort::toposort;

pub fn scc(adjacency_list: &[Vec<usize>]) -> Vec<usize> {
    let g = adjacency_list;

    let n = g.len();

    let mut order = vec![n; n];

    let mut labels = vec![n; n];

    let mut ord = 0;

    let mut label = 0;

    let mut preorder = Vec::with_capacity(n);

    let mut low = Vec::with_capacity(n);

    let mut st_dfs: Vec<isize> = (0..n as isize).rev().collect();

    while let Some(u) = st_dfs.pop() {
        if u < 0 {
            let u = !u as usize;

            if low.last().unwrap() != &u {
                continue;
            }

            loop {
                let v = preorder.pop().unwrap();

                labels[v] = label;

                if v == u {
                    break;
                }
            }

            label += 1;

            low.pop();

            continue;
        }

        let u = u as usize;

        if order[u] != n {
            continue;
        }

        st_dfs.push(!(u as isize));

        order[u] = ord;

        ord += 1;

        preorder.push(u);

        low.push(u);

        for &v in g[u].iter() {
            if order[v] == n {
                st_dfs.push(v as isize);
            } else if labels[v] == n {
                while order[*low.last().unwrap()] > order[v] {
                    low.pop();
                }
            }
        }
    }

    toposort(labels)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            (6, vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)]),
            vec![3, 1, 2, 3, 1, 0],
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
