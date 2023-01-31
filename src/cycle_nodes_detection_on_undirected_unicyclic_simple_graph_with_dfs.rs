// //! on undirected unicyclic simple graph.
// //! detect each node is in the cycle or not.
// pub fn detect_cycle(g: &[Vec<usize>]) -> Vec<bool> {
//     let n = g.len();
//     let mut on_cycle = vec![false; n];
//     let mut parent = vec![n; n];
//     parent[0] = 0;
//     fn dfs(
//         g: &[Vec<usize>], on_cycle: &mut [bool], parent: &mut [usize], u:
// usize,     ) {
//         let n = g.len();
//         for &v in g[u].iter() {
//             if v == parent[u] {
//                 continue;
//             }
//             if parent[v] == n {
//                 parent[v] = u;
//                 dfs(g, on_cycle, parent, v);
//                 continue;
//             }
//             on_cycle[u] = true;
//             let mut u = u;
//             while u != v {
//                 u = parent[u];
//                 on_cycle[u] = true;
//             }
//             break;
//         }
//     }
//     dfs(g, &mut on_cycle, &mut parent, 0);
//     on_cycle
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test() {
//         let g = vec![
//             vec![1, 2],
//             vec![0, 2],
//             vec![0, 1, 3, 4],
//             vec![2],
//             vec![2, 5, 6],
//             vec![4],
//             vec![4],
//         ];
//         let on_cycle = detect_cycle(&g);
//         assert_eq!(on_cycle, [true, true, true, false, false, false, false]);
//     }
// }
