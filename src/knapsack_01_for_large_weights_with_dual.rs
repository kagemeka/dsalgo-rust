use crate::knapsack_01_dual_table_with_inf::knapsack_dual;

pub fn knapsack(
    inf: usize,
    vw: &[(usize, usize)],
    max_weight: usize,
) -> usize {
    knapsack_dual(inf, vw)
        .into_iter()
        .enumerate()
        .filter_map(|(v, w)| if w <= max_weight { Some(v) } else { None })
        .max()
        .unwrap()
}
