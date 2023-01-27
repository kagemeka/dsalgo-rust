use crate::strongly_connected_components_topological_sort::toposort;

/// essentially same as Tarjan's Lowlink algorithm

pub fn scc(g: &[Vec<usize>]) -> Vec<usize> {
    fn labeling(
        g: &[Vec<usize>],
        preorder: &mut Vec<usize>,
        low: &mut Vec<usize>,
        order: &mut [usize],
        labels: &mut [usize],
        ord: &mut usize,
        label: &mut usize,
        u: usize,
    ) {
        order[u] = *ord;

        *ord += 1;

        preorder.push(u);

        low.push(u);

        let n = g.len();

        for v in g[u].to_owned().into_iter() {
            if order[v] == n {
                labeling(g, preorder, low, order, labels, ord, label, v);
            } else if labels[v] == n {
                while order[*low.last().unwrap()] > order[v] {
                    low.pop();
                }
            }
        }

        if *low.last().unwrap() != u {
            return;
        }

        loop {
            let v = preorder.pop().unwrap();

            labels[v] = *label;

            if v == u {
                break;
            }
        }

        *label += 1;

        low.pop();
    }

    let n = g.len();

    let mut order = vec![n; n];

    let mut labels = vec![n; n];

    let mut ord = 0;

    let mut label = 0;

    let mut preorder = Vec::with_capacity(n);

    let mut low = Vec::with_capacity(n);

    for i in 0..n {
        if order[i] == n {
            labeling(
                &g,
                &mut preorder,
                &mut low,
                &mut order,
                &mut labels,
                &mut ord,
                &mut label,
                i,
            );
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
