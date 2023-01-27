/// T is a numeric type.
/// if there are negative edges,
/// the correctness of the answer is not guaranteed.

pub fn dijkstra<T>(
    adj_list: &[Vec<(usize, T)>],
    s: usize,
) -> Vec<Option<T>>
where
    T: Copy + Ord + std::ops::Add<Output = T> + Default,
{
    use std::{
        cmp::Reverse,
        collections::BinaryHeap,
    };

    let g = adj_list;

    let n = g.len();

    let mut dist: Vec<Option<T>> = vec![None; n];

    dist[s] = Some(T::default());

    let mut q = BinaryHeap::new();

    q.push(Reverse((dist[s].unwrap(), s)));

    while let Some(Reverse((du, u))) = q.pop() {
        if Some(du) > dist[u] {
            continue;
        }

        for &(v, w) in &g[u] {
            let dv = du + w;

            if dist[v].is_some() && Some(dv) >= dist[v] {
                continue;
            }

            dist[v] = Some(dv);

            q.push(Reverse((dv, v)));
        }
    }

    dist
}
