/// return (parents, depths)

pub fn bfs(
    g: &[Vec<usize>],
    root: usize,
) -> (Vec<usize>, Vec<usize>) {
    let n = g.len();

    let mut p = vec![n; n];

    p[root] = root;

    let mut d = vec![0; n];

    let mut que = std::collections::VecDeque::new();

    que.push_back(root);

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if v == p[u] {
                continue;
            }

            p[v] = u;

            d[v] = d[u] + 1;

            que.push_back(v);
        }
    }

    (p, d)
}
