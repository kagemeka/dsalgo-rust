use crate::{
    dijkstra_sparse_queue::DijkstraSparseQueue,
    graph_edge_trait::{To, Weight},
};

pub fn general_dijkstra_sparse<E, T, F, Q>(
    sparse_graph: &[Vec<E>],
    update: &F,
    mut data: T,
    src: usize,
) -> T
where
    E: To<V = usize> + Weight<u64>,
    F: Fn(&mut T, usize, &E),
    Q: DijkstraSparseQueue,
{
    use std::cmp::Reverse;
    let n = sparse_graph.len();
    let mut hq = Q::default();
    let mut dist = vec![None; n];
    dist[src] = Some(0);
    hq.push(Reverse((0, src)));
    while let Some(Reverse((du, u))) = hq.pop() {
        if du > dist[u].unwrap() {
            continue;
        }
        for e in sparse_graph[u].iter() {
            update(&mut data, u, e);
            let v = *e.to();
            let dv = du + e.weight();
            if dist[v].is_some() && dv >= dist[v].unwrap() {
                continue;
            }
            dist[v] = Some(dv);
            hq.push(Reverse((dv, v)));
        }
    }
    data
}

// TODO
// test on each specific problems
// like sssp, path-count, and predecessors, ....
#[cfg(test)]
mod tests {}