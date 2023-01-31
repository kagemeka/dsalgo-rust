use crate::segment_tree_lazy_with_instance_homomorphism::*;

pub struct RangeXorRangeInversion01;

impl Ops for RangeXorRangeInversion01 {
    type F = i64;

    type S = (i64, i64, i64);

    // cnt_0, cnt_1, inversion
    fn compose(
        &self,
        f: Self::F,
        g: Self::F,
    ) -> Self::F {
        f ^ g
    }

    fn id(&self) -> Self::F { 0 }

    fn e(&self) -> Self::S { (0, 0, 0) }

    fn map(
        &self,
        f: Self::F,
        x: Self::S,
    ) -> Self::S {
        if f == 0 {
            x
        } else {
            (x.1, x.0, x.0 * x.1 - x.2)
        }
    }

    fn op(
        &self,
        a: Self::S,
        b: Self::S,
    ) -> Self::S {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2 + a.1 * b.0)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://atcoder.jp/contests/practice2/tasks/practice2_l
        let cases = vec![(
            vec![0, 1, 0, 0, 1],
            vec![
                ((2, 1, 5), 2),
                ((1, 3, 4), -1),
                ((2, 2, 5), 0),
                ((1, 1, 3), -1),
                ((2, 1, 2), 1),
            ],
        )];

        for (a, q) in cases {
            let n = a.len();

            let mut seg = LazySegtree::new(RangeXorRangeInversion01 {}, n);

            for i in 0..n {
                seg.set(i, if a[i] == 0 { (1, 0, 0) } else { (0, 1, 0) });
            }

            for ((t, l, r), ans) in q {
                if t == 1 {
                    seg.apply(l - 1, r, 1);
                } else {
                    assert_eq!(seg.fold(l - 1, r).2, ans);
                }
            }
        }
    }
}
