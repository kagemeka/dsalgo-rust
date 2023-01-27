//! from any node

use crate::functional_graph_doubling_table::*;

pub struct FunctionalGraphKthFrom(Vec<Vec<usize>>);

impl FunctionalGraphKthFrom {
    /// k <= 2^max_exp

    pub fn new(
        f: &[usize],
        max_exp: usize,
    ) -> Self {
        Self(doubling_table(f, max_exp))
    }

    pub fn get(
        &self,
        src: usize,
        k: usize,
    ) -> usize {
        let mut i = src;

        for (j, f) in self.0.iter().enumerate() {
            if k >> j & 1 == 1 {
                i = f[i];
            }
        }

        i
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let cases = vec![
            ((vec![1, 2, 0, 1, 5, 4], 3, 5), 2),
            ((vec![1, 2, 3, 4, 2, 1, 3, 4], 0, 1), 1),
        ];

        for ((f, src, k), ans) in cases {
            let f = FunctionalGraphKthFrom::new(&f, 10);

            assert_eq!(f.get(src, k), ans);
        }
    }
}
