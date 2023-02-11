pub struct Dsu {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self { root: (0..n).collect(), rank: vec![0; n] }
    }

    pub fn find_root(&self, u: usize) -> usize {
        let r = self.root[u];
        if r == u {
            u
        } else {
            self.find_root(r)
        }
    }

    pub fn unite(&mut self, mut u: usize, mut v: usize) {
        u = self.find_root(u);
        v = self.find_root(v);
        if u == v {
            return;
        }
        if self.rank[u] < self.rank[v] {
            std::mem::swap(&mut u, &mut v);
        }
        if self.rank[u] == self.rank[v] {
            self.rank[u] += 1;
        }
        self.root[v] = u;
    }
}
