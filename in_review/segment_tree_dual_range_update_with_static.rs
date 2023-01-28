use std::marker::PhantomData;

use crate::segment_tree_dual_with_static_monoid::*;

pub struct Update<T>(PhantomData<T>);

impl<T> Monoid for Update<T> {
    type T = Option<T>;

    fn e() -> Self::T { None }

    fn op(
        l: Self::T,
        r: Self::T,
    ) -> Self::T {
        if r.is_none() {
            l
        } else {
            r
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let mut seg = DualSegtree::<Update<usize>>::new(3);

        seg.operate(0, 2, Some(1));

        assert_eq!(seg.get(0), &Some(1));

        assert_eq!(seg.get(1), &Some(1));

        assert_eq!(seg.get(2), &None);

        seg.operate(1, 3, Some(2));

        assert_eq!(seg.get(0), &Some(1));

        assert_eq!(seg.get(1), &Some(2));

        assert_eq!(seg.get(2), &Some(2));

        seg.operate(2, 3, Some(3));

        assert_eq!(seg.get(0), &Some(1));

        assert_eq!(seg.get(1), &Some(2));

        assert_eq!(seg.get(2), &Some(3));
    }
}
