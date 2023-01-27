use crate::strongly_connected_components_transpose::transpose;

pub fn scc(g: &[Vec<usize>]) -> Vec<usize> {
    fn calc_topological_rev_ord(
        g: &[Vec<usize>],
        state: &mut [usize],
        post_order: &mut Vec<usize>,
        u: usize,
    ) {
        let n = g.len();

        state[u] = n;

        for &v in g[u].iter() {
            if state[v] == 0 {
                calc_topological_rev_ord(g, state, post_order, v);
            }
        }

        post_order.push(u);
    }

    fn labeling(
        g: &[Vec<usize>],
        labels: &mut [usize],
        l: usize,
        u: usize,
    ) {
        labels[u] = l;

        for &v in g[u].iter() {
            if labels[v] == g.len() {
                labeling(g, labels, l, v);
            }
        }
    }

    let n = g.len();

    let mut state = vec![0; n];

    let mut post_order = Vec::with_capacity(n);

    for i in 0..n {
        if state[i] == 0 {
            calc_topological_rev_ord(g, &mut state, &mut post_order, i);
        }
    }

    let g = transpose(g);

    let mut l = 0;

    for i in post_order.into_iter().rev() {
        if state[i] == n {
            labeling(&g, &mut state, l, i);

            l += 1;
        }
    }

    state
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
