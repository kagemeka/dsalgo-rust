pub fn connected_components(g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();

    let mut ids = vec![n; n];

    let mut id = 0;

    fn dfs(
        g: &[Vec<usize>],
        ids: &mut Vec<usize>,
        id: usize,
        u: usize,
    ) {
        ids[u] = id;

        for &v in g[u].iter() {
            if ids[v] == g.len() {
                dfs(g, ids, id, v);
            }
        }
    }

    for i in 0..n {
        if ids[i] != n {
            continue;
        }

        dfs(&g, &mut ids, id, i);

        id += 1;
    }

    ids
}
