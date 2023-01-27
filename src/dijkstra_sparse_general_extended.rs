use crate::{
    graph_edge_trait::{
        To,
        Weight,
    },
    priority_queue_trait::{
        Pop,
        Push,
    },
};

/// T is a numeric type.

pub fn dijkstra<Q, T, E, F>(
    adj_list: &[Vec<E>],
    src: usize,
    callback: &mut F,
) -> Vec<Option<T>>
where
    Q: Push<T = (T, usize)> + Pop<T = Option<(T, usize)>> + Default,
    T: Default + Ord + Copy + std::ops::Add<Output = T>,
    E: To<V = usize> + Weight<T = T>,
    F: FnMut(&Vec<Option<T>>, usize, &E),
{
    let g = adj_list;

    let n = g.len();

    let mut hq = Q::default();

    let mut dist: Vec<Option<T>> = vec![None; n];

    dist[src] = Some(T::default());

    hq.push((dist[src].unwrap(), src));

    while let Some((du, u)) = hq.pop() {
        if du > dist[u].unwrap() {
            continue;
        }

        for e in g[u].iter() {
            callback(&dist, u, e);

            let v = *e.to();

            let dv = du + *e.weight();

            if dist[v].is_some() && Some(dv) >= dist[v] {
                continue;
            }

            dist[v] = Some(dv);

            hq.push((dv, v));
        }
    }

    dist
}
