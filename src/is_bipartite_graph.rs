use crate::label_bipartite_graph_with_bfs::label_bipartite;

pub fn is_bipartite(g: &[Vec<usize>]) -> bool { label_bipartite(g).is_some() }

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        // TODO:
    }
}
