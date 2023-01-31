//! euler tour teqnique

use crate::{
    tree_bfs_parent_with_abstract::tree_parents,
    tree_edges_to_graph::tree_edges_to_graph,
};

/// Undirected Tree Edges.

pub type E = [(usize, usize)];

/// from edges, root

pub fn tour_edges(
    e: &E,
    r: usize,
) -> Vec<isize> {
    let g = tree_edges_to_graph(e);

    let n = g.len();

    let mut p = vec![None; n]; // parent
    let mut t = Vec::with_capacity(n << 1);

    let mut s = vec![r as isize]; // stack
    for _ in 0..n << 1 {
        let u = s.pop().unwrap();

        t.push(u);

        if u < 0 {
            continue;
        }

        s.push(!u);

        let u = u as usize;

        g[u].iter().rev().for_each(|&v| {
            if Some(v) != p[u] {
                p[v] = Some(u);

                s.push(v as isize);
            }
        });
    }

    t
}

// TODO: recurse
// pub fn tour_edges_recurse(e: &E, r: usize) -> Vec<isize> {}
pub fn tour_nodes(
    e: &E,
    r: usize,
) -> Vec<usize> {
    let p = tree_parents(e, r);

    tour_edges(e, r)
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(|&u| if u < 0 { p[!u as usize].unwrap() } else { u as usize })
        .collect()
}

pub fn last_positions(tour_nodes: &[usize]) -> Vec<usize> {
    let n = tour_nodes.iter().max().unwrap() + 1;

    let mut pos = vec![None; n];

    tour_nodes.iter().enumerate().for_each(|(i, &u)| pos[u] = Some(i));

    pos.iter().map(|&p| p.unwrap()).collect()
}

pub fn first_positions(tour_nodes: &[usize]) -> Vec<usize> {
    let size = tour_nodes.len();

    let mut tour = tour_nodes.to_vec();

    tour.reverse();

    last_positions(&tour).iter().map(|&i| size - i - 1).collect()
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
