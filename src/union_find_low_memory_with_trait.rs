//! Disjoint-Set-Union (DSU) or Union-Find (UF).

use crate::union_find_traits::*;

#[derive(Debug)]

pub struct UnionFind(Vec<isize>); // root: neg-size, other: parent
impl UnionFind {
    pub fn new(size: usize) -> Self { Self(vec![-1; size]) }
}

impl Size for UnionFind {
    fn size(&self) -> usize { self.0.len() }
}

impl Root for UnionFind {
    fn root(
        &mut self,
        u: usize,
    ) -> usize {
        if self.0[u] < 0 {
            return u;
        }

        self.0[u] = self.root(self.0[u] as usize) as isize;

        self.0[u] as usize
    }
}

impl Unite for UnionFind {
    fn unite(
        &mut self,
        u: usize,
        v: usize,
    ) {
        let mut u = self.root(u);

        let mut v = self.root(v);

        if u == v {
            return;
        }

        if self.0[u] > self.0[v] {
            std::mem::swap(&mut u, &mut v);
        }

        self.0[u] += self.0[v];

        self.0[v] = u as isize;
    }
}

impl SizeOf for UnionFind {
    fn size_of(
        &mut self,
        u: usize,
    ) -> usize {
        let u = self.root(u);

        -self.0[u] as usize
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_uf() {
        let mut uf = UnionFind::new(10);

        assert_eq!(uf.size_of(0), 1);

        uf.unite(3, 9);

        assert_eq!(uf.size_of(3), 2);
    }
}
