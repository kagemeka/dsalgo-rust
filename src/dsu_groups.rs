use crate::dsu_labels::labels;
use crate::dsu_trait::FindRoot;
use crate::size_trait::Size;

pub fn groups<T: FindRoot + Size>(uf: &mut T) -> Vec<Vec<usize>> {
    let lb = labels(uf);
    let k = *lb.iter().max().unwrap() + 1;
    let mut g = vec![vec![]; k];
    for (i, l) in lb.iter().enumerate() {
        g[*l].push(i);
    }
    g
}
