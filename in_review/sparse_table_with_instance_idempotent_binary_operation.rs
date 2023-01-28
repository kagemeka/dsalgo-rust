pub struct SparseTable<T, F> {
    node: Vec<Vec<T>>,
    f: F,
}

impl<T: Clone, F: Fn(T, T) -> T> SparseTable<T, F> {
    pub fn new(
        f: F,
        a: &[T],
    ) -> Self {
        let n = a.len();

        assert!(n > 0);

        let h = n.next_power_of_two().trailing_zeros().max(1) as usize;

        let mut node = vec![vec![]; h];

        node[0] = a.to_vec();

        for i in 1..h {
            let d1 = 1 << i;

            let d0 = d1 >> 1;

            let w = n - d1 + 1;

            node[i] = (0..w)
                .map(|j| f(node[i - 1][j].clone(), node[i - 1][j + d0].clone()))
                .collect();
        }

        Self { node, f }
    }

    pub fn size(&self) -> usize { self.node[0].len() }

    pub fn get(
        &self,
        l: usize,
        r: usize,
    ) -> T {
        assert!(l < r && r <= self.size());

        if r - l == 1 {
            return self.node[0][l].clone();
        }

        let i = (r - l).next_power_of_two().trailing_zeros() as usize - 1;

        (self.f)(self.node[i][l].clone(), self.node[i][r - (1 << i)].clone())
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a: Vec<usize> = vec![0, 4, 2, 8, 5, 1];

        let f = |a: usize, b: usize| a.min(b);

        let sp = SparseTable::new(&f, &a);

        assert_eq!(sp.get(0, 4), 0);

        assert_eq!(sp.get(3, 4), 8);

        assert_eq!(sp.get(1, 6), 1);
    }
}
