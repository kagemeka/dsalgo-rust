use crate::{
    abstract_traits::{AbelianGroup, Additive, Commutative, Monoid},
    bitwise,
};
/// Node Indices
/// (case $|given array| = 8$,
/// internally 1-indexed implemetation)
/// |8              |
/// |4      |       |
/// |2  |   |6  |   |
/// |1| |3| |5| |7| |
pub struct FenwickTree<M: Monoid<S, T> + Commutative<S, T>, S = M, T = Additive> {
    phantom_t: std::marker::PhantomData<T>,
    phantom_m: std::marker::PhantomData<M>,
    data: Vec<S>,
}

impl<M: Monoid<S, T> + Commutative<S, T>, S: Clone, T> From<&[S]> for FenwickTree<M, S, T> {
    fn from(slice: &[S]) -> Self {
        let size = slice.len();
        let mut data = vec![M::identity(); size + 1];
        data[1..].clone_from_slice(slice);
        for node_index in 1..size as isize {
            let parent_node_index = (node_index + (node_index & -node_index)) as usize;
            if parent_node_index <= size {
                data[parent_node_index] =
                    M::operate(&data[parent_node_index], &data[node_index as usize]);
            }
        }
        Self {
            phantom_t: std::marker::PhantomData,
            phantom_m: std::marker::PhantomData,
            data,
        }
    }
}

impl<M: Monoid<S, T> + Commutative<S, T>, S, T> FenwickTree<M, S, T> {
    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        (&vec![M::identity(); size]).as_slice().into()
    }

    pub fn size(&self) -> usize { self.data.len() - 1 }

    pub fn set_point(&mut self, array_index: usize, value_to_operate: &S) {
        assert!(array_index < self.size());
        let mut node_index = array_index + 1;
        while node_index <= self.size() {
            self.data[node_index] = M::operate(&self.data[node_index], value_to_operate);
            node_index += bitwise::lsb_number(node_index);
        }
    }

    pub fn get_half_range(&self, right: usize) -> S {
        assert!(right <= self.size());
        let mut value = M::identity();
        let mut node_index = right;
        while node_index > 0 {
            value = M::operate(&value, &self.data[node_index]);
            node_index = bitwise::reset_least_bit(node_index);
        }
        value
    }

    pub fn find_max_right<F>(&self, is_ok: &F) -> usize
    where
        F: Fn(&S) -> bool,
    {
        if self.size() == 0 {
            return 0;
        }
        let mut length = 1usize << bitwise::most_significant_bit(self.size()).unwrap();
        let mut value = M::identity();
        let mut right = 0;
        while length > 0 {
            if right + length <= self.size()
                && is_ok(&M::operate(&value, &self.data[right + length]))
            {
                right += length;
                value = M::operate(&value, &self.data[right]);
            }
            length >>= 1;
        }
        right
    }
}

impl<G: AbelianGroup<S, T>, S, T> FenwickTree<G, S, T> {
    pub fn get_range(&self, left: usize, right: usize) -> S {
        assert!(left <= right);
        G::operate(
            &G::invert(&self.get_half_range(left)),
            &self.get_half_range(right),
        )
    }

    pub fn get_point(&self, array_index: usize) -> S {
        assert!(array_index < self.size());
        self.get_range(array_index, array_index + 1)
    }

    pub fn find_max_right_with_left<F>(&self, is_ok: &F, left: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(left <= self.size());
        if left == self.size() {
            return self.size();
        }
        let mut length = 1usize << bitwise::most_significant_bit(self.size()).unwrap();
        let mut value = G::invert(&self.get_half_range(left));
        let mut right = 0;
        while length > 0 {
            if right + length <= left
                || right + length <= self.size()
                    && is_ok(&G::operate(&value, &self.data[right + length]))
            {
                right += length;
                value = G::operate(&value, &self.data[right]);
            }
            length >>= 1;
        }
        right
    }

    pub fn find_min_left_with_right<F>(&self, is_ok: &F, right: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(right <= self.size());
        if right == 0 {
            return 0;
        }
        let mut length = 1usize << bitwise::most_significant_bit(self.size()).unwrap();
        let mut value = self.get_half_range(right);
        if is_ok(&value) {
            return 0;
        }
        let mut left = 1;
        while length > 0 {
            if left + length <= right
                && !is_ok(&G::operate(&G::invert(&self.data[left - 1 + length]), &value))
            {
                left += length;
                value = G::operate(&G::invert(&self.data[left - 1]), &value);
            }
            length >>= 1;
        }
        left
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_as_abelian_group() {
        use crate::abstract_traits::{BinaryOperation, Commutative, Identity, Inverse};

        struct Add;
        impl Identity<Self, Add> for i32 {
            fn identity() -> Self { 0 }
        }
        impl BinaryOperation<Self, Add> for i32 {
            fn operate(x: &Self, y: &Self) -> Self { x + y }
        }
        impl Commutative<Self, Add> for i32 {}
        impl Inverse<Self, Add> for i32 {
            fn invert(value: &Self) -> i32 { -value }
        }

        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut fw = super::FenwickTree::<i32, i32, Add>::from(arr.as_slice());

        assert_eq!(fw.get_range(0, 10), 45);
        assert_eq!(fw.get_range(6, 10), 30);
        fw.set_point(5, &10);
        assert_eq!(fw.get_range(6, 10), 30);
        assert_eq!(fw.get_half_range(5), 10);
        assert_eq!(fw.get_half_range(6), 25);
        assert_eq!(fw.get_point(5), 15);
        let is_ok = |x: &i32| *x <= 25;
        assert_eq!(fw.find_max_right(&is_ok), 6);
        assert_eq!(fw.find_max_right_with_left(&is_ok, 0), 6);
        let is_ok = |x: &i32| *x < 25;
        assert_eq!(fw.find_max_right(&is_ok), 5);
        assert_eq!(fw.find_max_right_with_left(&is_ok, 0), 5);
        assert_eq!(fw.find_max_right_with_left(&is_ok, 4), 6);
        assert_eq!(fw.find_max_right_with_left(&is_ok, 5), 7);
        assert_eq!(fw.find_max_right_with_left(&is_ok, 6), 9);
        assert_eq!(fw.find_max_right_with_left(&is_ok, 9), 10);
        assert_eq!(fw.find_min_left_with_right(&is_ok, 10), 7);
        assert_eq!(fw.find_min_left_with_right(&is_ok, 0), 0);
        assert_eq!(fw.find_min_left_with_right(&is_ok, 6), 2);
        assert_eq!(fw.find_min_left_with_right(&is_ok, 5), 0);
        let is_ok = |x: &i32| *x < 15;
        assert_eq!(fw.find_max_right_with_left(&is_ok, 5), 5);
        assert_eq!(fw.find_min_left_with_right(&is_ok, 6), 6);
        assert_eq!(fw.find_min_left_with_right(&is_ok, 10), 9);
        let is_ok = |x: &i32| *x < 9;
        assert_eq!(fw.find_min_left_with_right(&is_ok, 10), 10);
    }
}
