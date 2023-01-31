//! compute connected components on undirected graphs.

pub fn connected_components(g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();

    let mut ids = vec![n; n];

    let mut id = 0;

    let mut que = std::collections::VecDeque::new();

    for i in 0..n {
        if ids[i] != n {
            continue;
        }

        ids[i] = id;

        que.push_back(i);

        while let Some(u) = que.pop_front() {
            for &v in &g[u] {
                if ids[v] != n {
                    continue;
                }

                ids[v] = id;

                que.push_back(v);
            }
        }

        id += 1;
    }

    ids
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
