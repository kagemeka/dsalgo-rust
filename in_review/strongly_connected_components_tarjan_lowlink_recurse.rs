use crate::strongly_connected_components_topological_sort::toposort;

pub fn scc(g: &[Vec<usize>]) -> Vec<usize> {
    fn labeling(
        g: &[Vec<usize>],
        preorder: &mut Vec<usize>,
        low_order: &mut Vec<usize>,
        order: &mut [usize],
        labels: &mut [usize],
        ord: &mut usize,
        label: &mut usize,
        u: usize,
    ) {
        order[u] = *ord;

        *ord += 1;

        preorder.push(u);

        let n = g.len();

        for &v in g[u].to_owned().iter() {
            if order[v] == n {
                labeling(g, preorder, low_order, order, labels, ord, label, v);

                low_order[u] = low_order[u].min(low_order[v]);
            } else if labels[v] == n {
                low_order[u] = low_order[u].min(order[v]);
            }
        }

        if low_order[u] < order[u] {
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
    }

    let n = g.len();

    let mut preorder = Vec::with_capacity(n);

    let mut low_order = vec![n; n];

    let mut order = vec![n; n];

    let mut labels = vec![n; n];

    let mut ord = 0;

    let mut label = 0;

    for i in 0..n {
        if order[i] == n {
            labeling(
                &g,
                &mut preorder,
                &mut low_order,
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
