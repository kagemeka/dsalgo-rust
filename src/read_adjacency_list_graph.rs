use crate::{
    adjacency_list_graph_from_edges::*,
    read_edges::read_edges,
};

/// read adjacency list

pub fn read_graph(
    n: usize,
    m: usize,
    offset: usize,
    directed: bool,
) -> Vec<Vec<usize>> {
    graph_from_edges(n, &read_edges(m, offset), directed)
}
