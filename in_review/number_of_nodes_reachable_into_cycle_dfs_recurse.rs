pub fn check_into_cycle(g: &[Vec<usize>]) -> Vec<bool> {
    let n = g.len();

    let mut to_cycle = vec![false; n];

    let mut state = vec![0; n];

    fn dfs(
        g: &[Vec<usize>],
        to_cycle: &mut [bool],
        state: &mut [u8],
        u: usize,
    ) {
        state[u] = 1;

        for &v in g[u].iter() {
            if state[v] == 0 {
                dfs(g, to_cycle, state, v);
            } else if state[v] == 1 {
                to_cycle[u] = true;
            }

            to_cycle[u] |= to_cycle[v];
        }

        state[u] = 2;
    }

    for i in 0..n {
        if state[i] == 0 {
            dfs(g, &mut to_cycle, &mut state, i);
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
