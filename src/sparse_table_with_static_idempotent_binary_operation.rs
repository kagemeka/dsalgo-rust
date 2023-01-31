pub trait BinaryOp {
    type T;

    fn f(
        _: Self::T,
        _: Self::T,
    ) -> Self::T;
}

pub struct SparseTable<F: BinaryOp>(Vec<Vec<F::T>>);

impl<F: BinaryOp> SparseTable<F>
where
    F::T: Clone,
{
    pub fn new(a: &[F::T]) -> Self {
        let n = a.len();

        assert!(n > 0);

        let h = n.next_power_of_two().trailing_zeros().max(1) as usize;

        let mut data = vec![vec![]; h];

        data[0] = a.to_vec();

        for i in 1..h {
            let d1 = 1 << i;

            let d0 = d1 >> 1;

            let w = n - d1 + 1;

            data[i] = (0..w)
                .map(|j| {
                    F::f(data[i - 1][j].clone(), data[i - 1][j + d0].clone())
                })
                .collect();
        }

        Self(data)
    }

    pub fn size(&self) -> usize { self.0[0].len() }

    pub fn get(
        &self,
        l: usize,
        r: usize,
    ) -> F::T {
        assert!(l < r && r <= self.size());

        if r - l == 1 {
            return self.0[0][l].clone();
        }

        let i = (r - l).next_power_of_two().trailing_zeros() as usize - 1;

        F::f(self.0[i][l].clone(), self.0[i][r - (1 << i)].clone())
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        struct F;

        impl BinaryOp for F {
            type T = usize;

            fn f(
                x: usize,
                y: usize,
            ) -> usize {
                x.min(y)
            }
        }

        let a: Vec<usize> = vec![0, 4, 2, 8, 5, 1];

        let sp = SparseTable::<F>::new(&a);

        assert_eq!(sp.get(0, 4), 0);

        assert_eq!(sp.get(3, 4), 8);

        assert_eq!(sp.get(1, 6), 1);
    }
}
