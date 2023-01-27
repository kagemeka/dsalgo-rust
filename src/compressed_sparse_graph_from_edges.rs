pub fn csgraph_from_edges(
    mut edges: Vec<(usize, usize)>
) -> Vec<(usize, usize)> {
    edges.sort_unstable();

    edges
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let edges = vec![(0, 2), (2, 1), (0, 1)];

        assert_eq!(csgraph_from_edges(edges), vec![(0, 1), (0, 2), (2, 1)]);
    }
}
