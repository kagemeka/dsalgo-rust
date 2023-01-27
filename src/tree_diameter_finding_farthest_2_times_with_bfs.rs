pub fn tree_diameter(g: &[Vec<usize>]) -> usize {
    let n = g.len();

    let mut que = std::collections::VecDeque::new();

    let mut bfs = |u: usize| {
        let mut depth = vec![n; n];

        depth[u] = 0;

        que.push_back(u);

        while let Some(u) = que.pop_front() {
            for &v in g[u].iter() {
                if depth[v] != n {
                    continue;
                }

                depth[v] = depth[u] + 1;

                que.push_back(v);
            }
        }

        depth
    };

    let mut d = 0;

    let mut u = 0;

    for _ in 0..2 {
        d = 0;

        let dep = bfs(u);

        for i in 0..n {
            if dep[i] <= d {
                continue;
            }

            d = dep[i];

            u = i;
        }
    }

    d
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let n = 10;

        let edges = vec![
            (6, 5),
            (2, 1),
            (1, 3),
            (3, 4),
            (7, 8),
            (0, 7),
            (0, 5),
            (0, 1),
            (8, 9),
        ];

        let mut g = vec![vec![]; n];

        for (u, v) in edges {
            g[u].push(v);

            g[v].push(u);
        }

        assert_eq!(tree_diameter(&g), 6);
    }
}
