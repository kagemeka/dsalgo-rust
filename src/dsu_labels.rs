use crate::dsu_trait::FindRoot;
use crate::size_trait::Size;
pub fn labels<T: FindRoot + Size>(uf: &mut T) -> Vec<usize> {
    let n = uf.size();
    let mut labels = vec![n; n];
    let mut l = 0;
    for i in 0..n {
        let r = uf.find_root(i);
        if labels[r] == n {
            labels[r] = l;
            l += 1;
        }
        labels[i] = labels[r];
    }
    labels
}
