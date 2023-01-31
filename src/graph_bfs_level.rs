pub fn bfs_level(
    g: &[Vec<usize>],
    src: usize,
) -> Vec<usize> {
    let n = g.len();

    let mut lv = vec![n; n];

    let mut que = std::collections::VecDeque::new();

    que.push_back(src);

    lv[src] = 0;

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if lv[v] != n {
                continue;
            }

            lv[v] = lv[u] + 1;

            que.push_back(v);
        }
    }

    lv
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
            0,
            vec![0, 1, 1, 2],
        )];

        for (g, src, ans) in cases {
            assert_eq!(bfs_level(&g, src), ans);
        }
    }
}
