pub fn transpose<T: Clone>(g: Vec<Vec<(usize, T)>>) -> Vec<Vec<(usize, T)>> {
    let n = g.len();

    let mut t = vec![vec![]; n];

    for (u, edges) in g.into_iter().enumerate() {
        for (v, w) in edges.into_iter() {
            t[v].push((u, w));
        }
    }

    t
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![vec![(1, 0), (2, 1)], vec![(2, 2)], vec![(0, 3)]];

        let t = vec![vec![(2, 3)], vec![(0, 0)], vec![(0, 1), (1, 2)]];

        assert_eq!(transpose(g), t);
    }
}
