pub fn bfs() {}

pub fn dfs() {}

use crate::graph::edge::{
    From,
    Reversed,
    To,
    ToDirected,
};

/// return edge ids.

pub fn dfs_arborescense<E>(
    v_size: usize,
    directed_edges: &[E],
    root: usize,
) -> Vec<usize>
where
    E: From<V = usize> + To<V = usize>,
{
    let mut g = vec![vec![]; v_size];

    for (id, e) in directed_edges.iter().enumerate() {
        let u = *e.from();

        let v = *e.to();

        g[u].push((v, id));
    }

    let mut visited = vec![false; v_size];

    let mut added_edges = vec![];

    fn dfs(
        g: &Vec<Vec<(usize, usize)>>,
        visited: &mut Vec<bool>,
        added_edges: &mut Vec<usize>,
        u: usize,
    ) {
        visited[u] = true;

        for &(v, i) in &g[u] {
            if visited[u] {
                continue;
            }

            added_edges.push(i);

            dfs(g, visited, added_edges, v);
        }
    }

    dfs(&g, &mut visited, &mut added_edges, root);

    added_edges
}

pub fn bfs_arborescence() {}

/// return edge ids

pub fn dfs_tree<E1, E2>(
    v_size: usize,
    undirected_edges: &[E1],
    root: usize,
) -> Vec<usize>
where
    E1: From<V = usize> + To<V = usize> + Clone + ToDirected<E = E2>,
    E2: Reversed + From<V = usize> + To<V = usize>,
{
    let m = undirected_edges.len();

    let edges = edges_to_directed(undirected_edges.to_vec());

    dfs_arborescense(v_size, &edges, root).into_iter().map(|i| i % m).collect()
}

pub fn bfs_tree() {}

// TODO: rename e_to_d
/// undirected edges to directed

pub fn edges_to_directed<UE, DE>(undirected_edges: Vec<UE>) -> Vec<DE>
where
    UE: Clone + ToDirected<E = DE>,
    DE: Reversed,
{
    let di_edges = undirected_edges.into_iter().map(|e| e.to_directed());

    let rev = di_edges.clone().map(|e| e.reversed());

    di_edges.chain(rev).collect()
}

// TODO
#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
