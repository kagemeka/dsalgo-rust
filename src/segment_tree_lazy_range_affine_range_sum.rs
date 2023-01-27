use std::{
    marker::PhantomData,
    ops::*,
};

use crate::segment_tree_lazy_with_instance_homomorphism::*;

pub struct RangeAffineRangeSum<T>(PhantomData<T>);

impl<T> RangeAffineRangeSum<T> {
    pub fn new() -> Self { Self(PhantomData) }
}

impl<T> Ops for RangeAffineRangeSum<T>
where
    T: From<i32> + Clone + Mul<Output = T> + Add<Output = T>,
{
    type F = (T, T);

    // (a, b)
    type S = (T, T);

    // (sum, len)
    fn compose(
        &self,
        f: Self::F,
        g: Self::F,
    ) -> Self::F {
        (f.0.clone() * g.0, f.0 * g.1 + f.1)
    }

    fn e(&self) -> Self::S { (0.into(), 0.into()) }

    fn op(
        &self,
        a: Self::S,
        b: Self::S,
    ) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }

    fn id(&self) -> Self::F { (1.into(), 0.into()) }

    fn map(
        &self,
        f: Self::F,
        x: Self::S,
    ) -> Self::S {
        (f.0 * x.0 + f.1 * x.1.clone(), x.1)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://atcoder.jp/contests/practice2/tasks/practice2_k
        use crate::const_generics_modular_int_i64::Modint;

        type Mint = Modint<998_244_353>;

        let cases = vec![(
            vec![1, 2, 3, 4, 5],
            vec![
                ((1, 0, 5, -1, -1), 15),
                ((0, 2, 4, 100, 101), -1),
                ((1, 0, 3, -1, -1), 404),
                ((0, 1, 3, 102, 103), -1),
                ((1, 2, 5, -1, -1), 41511),
                ((0, 2, 5, 104, 105), -1),
                ((1, 0, 5, -1, -1), 4317767),
            ],
        )];

        for (a, q) in cases {
            let n = a.len();

            let mut seg =
                LazySegtree::new(RangeAffineRangeSum::<Mint>::new(), n);

            for i in 0..n {
                seg.set(i, (a[i].into(), 1.into()));
            }

            for ((t, l, r, b, c), ans) in q {
                if t == 0 {
                    seg.apply(l, r, (b.into(), c.into()));
                } else {
                    assert_eq!(seg.fold(l, r).0 .0, ans);
                }
            }
        }
    }
}
