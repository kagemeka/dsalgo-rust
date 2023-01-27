//! bfs from multiple nodes in the same bfs. O(N + M)

pub fn bfs_reachable(
    g: &[Vec<usize>],
    sources: &[usize],
) -> Vec<bool> {
    let n = g.len();

    let mut reachable = vec![false; n];

    let mut que = std::collections::VecDeque::new();

    for &u in sources.iter() {
        que.push_back(u);

        reachable[u] = true;
    }

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if reachable[v] {
                continue;
            }

            reachable[v] = true;

            que.push_back(v);
        }
    }

    reachable
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![
            vec![5],
            vec![0],
            vec![1],
            vec![6],
            vec![],
            vec![],
            vec![],
            vec![4],
        ];

        let sources = vec![1, 6, 7];

        assert_eq!(
            bfs_reachable(&g, &sources),
            [true, true, false, false, true, true, true, true]
        );
    }
}
