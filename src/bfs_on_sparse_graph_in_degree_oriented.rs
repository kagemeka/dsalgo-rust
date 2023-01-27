pub fn bfs<F, H>(
    g: &[Vec<usize>],
    mut f: F,
    mut h: H,
) where
    F: FnMut(usize, usize),
    H: FnMut(usize),
{
    let n = g.len();

    let mut in_deg = vec![0; n];

    for u in 0..n {
        for &v in g[u].iter() {
            in_deg[v] += 1;
        }
    }

    let mut que = std::collections::VecDeque::new();

    for i in 0..n {
        if in_deg[i] == 0 {
            que.push_back(i);
        }
    }

    while let Some(u) = que.pop_front() {
        h(u);

        for &v in g[u].iter() {
            f(u, v);

            in_deg[v] -= 1;

            if in_deg[v] == 0 {
                que.push_back(v);
            }
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
