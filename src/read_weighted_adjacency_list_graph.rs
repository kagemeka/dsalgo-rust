use crate::{
    adjacency_list_graph_from_edges_weighted::graph_from_weigted_edges,
    read_weigted_edges::read_weighted_edges,
};

pub fn read_weigted_graph<T: std::str::FromStr + Clone>(
    n: usize,
    m: usize,
    offset: usize,
    directed: bool,
) -> Vec<Vec<(usize, T)>> {
    graph_from_weigted_edges(n, &read_weighted_edges(m, offset), directed)
}
