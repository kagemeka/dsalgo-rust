use crate::segment_tree_with_instance_monoid::*;

pub struct PrefixSumMinimum;

impl Monoid for PrefixSumMinimum {
    type T = (i64, i64);

    fn e(&self) -> Self::T { (std::i64::MAX, 0) }

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T {
        (if r.0 == std::i64::MAX { l.0 } else { l.0.min(l.1 + r.0) }, l.1 + r.1)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // https://atcoder.jp/contests/abc223/tasks/abc223_f
        let cases = vec![
            (
                vec![1, 1, -1, -1, 1],
                vec![((2, 0, 3), true), ((2, 0, 1), false), ((2, 3, 4), false)],
            ),
            (
                vec![1, 1, -1, -1, 1],
                vec![
                    ((2, 0, 3), true),
                    ((1, 0, 3), false), // if t == 1, false
                    ((2, 0, 3), false),
                ],
            ),
            (
                vec![1, 1, -1, 1, 1, -1, -1, -1],
                vec![
                    ((2, 1, 6), true),
                    ((2, 1, 7), false),
                    ((1, 1, 4), false),
                    ((2, 2, 3), false),
                    ((1, 2, 3), false),
                    ((1, 2, 4), false),
                    ((1, 0, 3), false),
                    ((1, 5, 7), false),
                ],
            ),
        ];

        for (mut a, queries) in cases {
            let n = a.len();

            let m = PrefixSumMinimum {};

            let mut seg = SegmentTree::new(m, n);

            for i in 0..n {
                seg.set(i, (a[i], a[i]));
            }

            for ((t, l, r), ans) in queries {
                if t == 1 {
                    a.swap(l, r);

                    seg.set(l, (a[l], a[l]));

                    seg.set(r, (a[r], a[r]));
                } else {
                    let s = seg.fold(l, r + 1);

                    assert_eq!(s.0 >= 0 && s.1 == 0, ans);
                }
            }
        }
    }
}
