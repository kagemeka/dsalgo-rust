use std::ops::*;

#[derive(Clone, PartialEq, Eq)]

pub struct Value<T>(Option<T>);

impl<T> Add for Value<T> {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        if rhs.0.is_none() {
            self
        } else {
            rhs
        }
    }
}

impl<T> From<i32> for Value<T> {
    fn from(x: i32) -> Self {
        assert!(x == 0);

        Self(None)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        use crate::segment_tree_dual_additive_from_i32_with_std::*;

        let mut seg = DualSegtree::new(3);

        seg.operate(0, 2, Value(Some(1)));

        assert_eq!(seg.get(0).0, Some(1));

        assert_eq!(seg.get(1).0, Some(1));

        assert_eq!(seg.get(2).0, None);

        seg.operate(1, 3, Value(Some(2)));

        assert_eq!(seg.get(0).0, Some(1));

        assert_eq!(seg.get(1).0, Some(2));

        assert_eq!(seg.get(2).0, Some(2));

        seg.operate(2, 3, Value(Some(3)));

        assert_eq!(seg.get(0).0, Some(1));

        assert_eq!(seg.get(1).0, Some(2));

        assert_eq!(seg.get(2).0, Some(3));
    }
}
