pub struct Dsu {
    root: Vec<usize>,
    size: Vec<usize>,
}
impl Dsu {
    pub fn new(n: usize) -> Self {
        Self { root: (0..n).collect(), size: vec![1; n] }
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
        if self.size[u] < self.size[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.size[u] += self.size[v];
        self.root[v] = u;
    }
}
