use crate::dsu_trait::FindRoot;

pub fn same<T: FindRoot>(uf: &mut T, u: usize, v: usize) -> bool {
    uf.find_root(u) == uf.find_root(v)
}
