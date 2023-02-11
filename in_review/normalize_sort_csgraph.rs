type G = Vec<(usize, usize)>;

pub fn normalize_csgraph(mut edges: G) -> G {
    edges.sort_by(|e0, e1| e0.1.cmp(&e1.1));

    edges.sort_by(|e0, e1| e0.0.cmp(&e1.0));

    edges
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let g = vec![(1, 4), (2, 1), (0, 3), (2, 0)];

        assert_eq!(normalize_csgraph(g), [(0, 3), (1, 4), (2, 0), (2, 1)]);
    }
}
