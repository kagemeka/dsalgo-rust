pub fn transpose(g: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = g.len();

    let mut t = vec![vec![]; n];

    for u in 0..n {
        for &v in g[u].iter() {
            t[v].push(u);
        }
    }

    t
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![vec![1, 2], vec![2], vec![0]];

        let t = vec![vec![2], vec![0], vec![0, 1]];

        assert_eq!(transpose(&g), t);
    }
}
