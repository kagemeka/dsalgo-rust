/// make undirected edges to bidirected.

pub fn edges_to_bidirected(edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    edges
        .to_vec()
        .into_iter()
        .chain(edges.to_vec().into_iter().map(|(u, v)| (v, u)))
        .collect()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        assert_eq!(edges_to_bidirected(&vec![(0, 1)]), vec![(0, 1), (1, 0)]);
    }
}
