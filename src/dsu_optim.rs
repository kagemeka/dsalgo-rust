pub struct Dsu {
    data: Vec<isize>,
}
impl Dsu {
    pub fn new(size: usize) -> Self {
        Self { data: vec![-1; size] }
    }
    pub fn root(&mut self, u: usize) -> usize {
        if self.data[u] < 0 {
            return u;
        }
        let r = self.root(self.data[u] as usize);
        self.data[u] = r as isize;
        r
    }
    pub fn unite(&mut self, mut u: usize, mut v: usize) {
        u = self.root(u);
        v = self.root(v);
        if u == v {
            return;
        }
        if self.data[u] > self.data[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.data[u] += self.data[v];
        self.data[v] = u as isize;
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn size_of(&mut self, u: usize) -> usize {
        let r = self.root(u);
        -self.data[r] as usize
    }
    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
    pub fn labels(&mut self) -> Vec<usize> {
        let n = self.size();
        let mut labels = vec![n; n];
        let mut l = 0;
        for i in 0..n {
            let r = self.root(i);
            if labels[r] == n {
                labels[r] = l;
                l += 1;
            }
            labels[i] = labels[r];
        }
        labels
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let labels = self.labels();
        let k = *labels.iter().max().unwrap() + 1;
        let mut g = vec![vec![]; k];
        for (i, l) in labels.iter().enumerate() {
            g[*l].push(i);
        }
        g
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut uf = Dsu::new(10);
        assert_eq!(uf.size_of(0), 1);
        uf.unite(3, 9);
        assert_eq!(uf.size_of(3), 2);
    }
    #[test]
    fn test_practice2_b() {
        let cases = vec![(
            4,
            vec![
                ((1, 0, 1), 0),
                ((0, 0, 1), 2),
                ((0, 2, 3), 2),
                ((1, 0, 1), 1),
                ((1, 1, 2), 0),
                ((0, 0, 2), 2),
                ((1, 1, 3), 1),
            ],
        )];
        for (n, q) in cases {
            let mut uf = Dsu::new(n);
            for ((t, u, v), ans) in q {
                if t == 0 {
                    uf.unite(u, v);
                } else {
                    assert_eq!(uf.same(u, v) as i32, ans);
                    // assert_eq!(if uf.same(u, v) { 1 } else { 0 }, ans);
                }
            }
        }
    }
}
