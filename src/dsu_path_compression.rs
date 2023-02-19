use crate::dsu::Dsu;
impl Dsu {
    pub fn root_compressing(&mut self, u: usize) -> usize {
        let mut r = self.root[u];
        if r == u {
            return u;
        }
        r = self.root_compressing(r);
        self.root[u] = r;
        r
    }
}
