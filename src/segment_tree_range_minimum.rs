use crate::segment_tree_with_instance_monoid::*;

pub struct RangeMinimum<T>(T);

impl<T> RangeMinimum<T> {
    pub fn new(inf: T) -> Self { Self(inf) }
}

impl<T: Ord + Clone> Monoid for RangeMinimum<T> {
    type T = T;

    fn e(&self) -> Self::T { self.0.clone() }

    fn op(
        &self,
        l: Self::T,
        r: Self::T,
    ) -> Self::T {
        l.min(r)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://atcoder.jp/contests/practice2/tasks/practice2_j
        let cases = vec![(
            vec![1, 2, 3, 2, 1],
            vec![
                ((2, 1, 5), 3),
                ((3, 2, 3), 3),
                ((1, 3, 1), -1),
                ((2, 2, 4), 2),
                ((3, 1, 3), 6),
            ],
        )];

        for (a, q) in cases {
            let n = a.len();

            let mut seg = SegmentTree::new(RangeMinimum::new(1 << 30), n);

            for i in 0..n {
                seg.set(i, -a[i]);
            }

            for ((t, x, y), ans) in q {
                if t == 1 {
                    seg.set(x as usize - 1, -y);
                } else if t == 2 {
                    assert_eq!(-seg.fold(x as usize - 1, y as usize), ans);
                } else {
                    assert_eq!(
                        seg.max_right(|v| v > &-y, x as usize - 1) + 1,
                        ans as usize,
                    );
                }
            }
        }
    }
}
