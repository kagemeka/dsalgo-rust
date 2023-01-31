/// labeling in postorder

pub fn hld(
    g: &[Vec<usize>],
    root: usize,
) -> Vec<usize> {
    fn dfs(
        g: &[Vec<usize>],
        size: &mut [usize],
        label: &mut [usize],
        l: &mut usize,
        u: usize,
        p: usize,
    ) {
        let n = g.len();

        for &v in g[u].iter() {
            if v == p {
                continue;
            }

            dfs(g, size, label, l, v, u);

            size[u] += size[v];
        }

        let mut heavy = n;

        for &v in g[u].iter() {
            if v == p {
                continue;
            }

            if size[v] << 1 >= size[u] {
                heavy = v;
            }
        }

        if heavy == n {
            label[u] = *l;

            *l += 1;
        } else {
            label[u] = label[heavy];
        }
    }

    let n = g.len();

    let mut label = vec![n; n];

    let mut l = 0;

    let mut size = vec![1; n];

    dfs(&g, &mut size, &mut label, &mut l, root, n);

    label
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (4, 5), (4, 6)];

        let n = 7;

        let mut g = vec![vec![]; n];

        for (u, v) in edges {
            g[u].push(v);

            g[v].push(u);
        }

        let label = hld(&g, 0);

        assert_eq!(label, [3, 3, 4, 0, 3, 1, 2,]);
    }
}
