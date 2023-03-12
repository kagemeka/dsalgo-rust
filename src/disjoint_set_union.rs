// https://en.wikipedia.org/wiki/Disjoint-set_data_structure
pub struct Dsu {
    pub root: Vec<usize>,
}
impl Dsu {
    pub fn new(n: usize) -> Self {
        Self { root: (0..n).collect() }
    }
    pub fn find_root(&self, u: usize) -> usize {
        let r = self.root[u];
        if r == u {
            u
        } else {
            self.find_root(r)
        }
    }
    pub fn unite(&mut self, u: usize, v: usize) {
        let u = self.find_root(u);
        let v = self.find_root(v);
        self.root[v] = u;
    }
}
