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

// TODO: generalize count type.
pub fn dijkstra_path_count<Q, T, E>(
    adj_list: &[Vec<E>],
    src: usize,
) -> (Vec<Option<T>>, Vec<u64>)
where
    Q: Push<T = (T, usize)> + Pop<T = Option<(T, usize)>> + Default,
    T: Default + Ord + Copy + std::ops::Add<Output = T>,
    E: To<V = usize> + Weight<T = T>,
{
    use crate::dijkstra_sparse_general_extended::dijkstra as f;

    const MOD: u64 = 1_000_000_007;

    let n = adj_list.len();

    let mut cnt = vec![0; n];

    cnt[src] = 1;

    let dist = f::<Q, _, _, _>(
        adj_list,
        src,
        &mut |d: &Vec<Option<T>>, u: usize, e: &E| {
            let v = *e.to();

            let dv = d[u].unwrap() + *e.weight();

            if d[v].is_none() || Some(dv) < d[v] {
                cnt[v] = cnt[u];
            } else if Some(dv) == d[v] {
                cnt[v] = (cnt[v] + cnt[u]) % MOD;
            }
        },
    );

    (dist, cnt)
}
