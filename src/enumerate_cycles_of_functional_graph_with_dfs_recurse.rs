pub fn enumerate_cycles(f: &[usize]) -> Vec<Vec<usize>> {
    let mut cycles = vec![];

    let n = f.len();

    let mut state = vec![0; n];

    fn dfs(
        f: &[usize],
        cycles: &mut Vec<Vec<usize>>,
        state: &mut Vec<usize>,
        u: usize,
    ) {
        if state[u] == 1 {
            let mut cycle = vec![];

            let mut v = u;

            loop {
                cycle.push(v);

                v = f[v];

                if v == u {
                    break;
                }
            }

            cycles.push(cycle);

            return;
        }

        state[u] = 1;

        let v = f[u];

        if state[v] != 2 {
            dfs(f, cycles, state, v);
        }

        state[u] = 2;
    }

    for i in 0..n {
        if state[i] == 0 {
            dfs(f, &mut cycles, &mut state, i);
        }
    }

    cycles
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            (vec![1, 0], vec![vec![0, 1]]),
            (vec![0, 0], vec![vec![0]]),
            (vec![0, 1, 2], vec![vec![0], vec![1], vec![2]]),
        ];

        for (f, ans) in cases {
            assert_eq!(enumerate_cycles(&f), ans);
        }
    }
}
