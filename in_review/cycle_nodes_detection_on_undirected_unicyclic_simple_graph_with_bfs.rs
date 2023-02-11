//! on undirected unicyclic simple graph.
//! detect each node is in the cycle or not.

pub fn detect_on_cycle(g: &[Vec<usize>]) -> Vec<bool> {
    let n = g.len();

    let mut on_cycle = vec![true; n];

    let mut in_deg = vec![0; n];

    for edges in g.iter() {
        for &v in edges.iter() {
            in_deg[v] += 1;
        }
    }

    let mut que = std::collections::VecDeque::new();

    for i in 0..n {
        if in_deg[i] == 1 {
            que.push_back(i);

            on_cycle[i] = false;
        }
    }

    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if !on_cycle[v] {
                continue;
            }

            in_deg[v] -= 1;

            if in_deg[v] == 1 {
                que.push_back(v);

                on_cycle[v] = false;
            }
        }
    }

    on_cycle
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![
            vec![1, 2],
            vec![0, 2],
            vec![0, 1, 3, 4],
            vec![2],
            vec![2, 5, 6],
            vec![4],
            vec![4],
        ];

        let on_cycle = detect_on_cycle(&g);

        assert_eq!(on_cycle, [true, true, true, false, false, false, false]);
    }
}
