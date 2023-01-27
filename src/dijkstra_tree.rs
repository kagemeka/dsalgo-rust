use crate::{
    dijkstra_arborescence::dijkstra_arborescense,
    dijkstra_sparse_queue::DijkstraSparseQueue,
    graph::edge::{
        From,
        Reversed,
        To,
        ToDirected,
        Weight,
    },
    graphops::edges_to_directed,
};

/// return edge ids

pub fn dijkstra_tree<E1, E2, Q>(
    v_size: usize,
    undirected_edges: &[E1],
    root: usize,
) -> Vec<usize>
where
    E1: From<V = usize> + To<V = usize> + Clone + ToDirected<E = E2>,
    E2: Reversed + From<V = usize> + To<V = usize> + Weight<u64> + Clone,
    Q: DijkstraSparseQueue,
{
    let m = undirected_edges.len();

    let edges = edges_to_directed(undirected_edges.to_vec());

    dijkstra_arborescense::<_, Q>(v_size, &edges, root)
        .into_iter()
        .map(|i| i % m)
        .collect()
}

// TODO
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
