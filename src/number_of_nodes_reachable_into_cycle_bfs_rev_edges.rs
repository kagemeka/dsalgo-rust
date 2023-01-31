pub fn check_into_cycle(g: &[Vec<usize>]) -> Vec<bool> {
    let n = g.len();

    let mut t = vec![vec![]; n];

    let mut in_deg = vec![0; n];

    for u in 0..n {
        for &v in g[u].iter() {
            t[v].push(u);

            in_deg[u] += 1;
        }
    }

    let mut que = std::collections::VecDeque::new();

    let mut to_cycle = vec![true; n];

    for i in 0..n {
        if in_deg[i] == 0 {
            que.push_back(i);

            to_cycle[i] = false;
        }
    }

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if !to_cycle[v] {
                continue;
            }

            in_deg[v] -= 1;

            if in_deg[v] == 0 {
                que.push_back(v);

                to_cycle[v] = false;
            }
        }
    }

    to_cycle
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![(
            vec![vec![1], vec![2], vec![3], vec![1, 4], vec![]],
            vec![true, true, true, true, false],
        )];

        for (g, ans) in cases {
            assert_eq!(check_into_cycle(&g), ans);
        }
    }
}
