use crate::abstract_traits::{Additive, Monoid};

fn merge<M: Monoid<S, T>, S, T>(data: &mut Vec<S>, node_index: usize) {
    data[node_index] = M::operate(&data[node_index << 1], &data[node_index << 1 | 1]);
}

fn make_data<M: Monoid<S, T>, S: Clone, T>(arr: &Vec<S>) -> Vec<S> {
    let size = arr.len();
    assert!(size > 0);
    let n = size.next_power_of_two();
    let mut data = vec![M::identity(); n << 1];
    data[n..(n + size)].clone_from_slice(arr);
    for node_index in (1..n).rev() {
        merge::<M, S, T>(&mut data, node_index);
    }
    data
}

fn set<M: Monoid<S, T>, S, T>(data: &mut Vec<S>, array_index: usize, x: S) {
    let mut node_index = array_index + (data.len() >> 1);
    data[node_index] = x;
    while node_index > 1 {
        node_index >>= 1;
        merge::<M, S, T>(data, node_index);
    }
}

/// Node Indices (case $4 \lt |given array| \le 8$)
/// |1                      |2
/// |2          |3          |4
/// |4    |5    |6    |7    |8
/// |8 |9 |10|11|12|13|14|15|16
pub struct SegmentTree<M: Monoid<S, T>, S = M, T = Additive> {
    phantom_t: std::marker::PhantomData<T>,
    phantom_m: std::marker::PhantomData<M>,
    size: usize,
    data: Vec<S>,
}

impl<M: Monoid<S, T>, S: Clone, T> From<&Vec<S>> for SegmentTree<M, S, T> {
    fn from(arr: &Vec<S>) -> Self {
        Self {
            phantom_t: std::marker::PhantomData,
            phantom_m: std::marker::PhantomData,
            size: arr.len(),
            data: make_data::<M, S, T>(arr),
        }
    }
}

impl<M: Monoid<S, T>, S, T> SegmentTree<M, S, T> {
    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        (&vec![M::identity(); size]).into()
    }

    pub fn set(&mut self, i: usize, x: S) {
        assert!(i < self.size);
        set::<M, S, T>(&mut self.data, i, x);
    }

    pub fn get(&self, left: usize, right: usize) -> S {
        assert!(left <= right && right <= self.size);
        let n = self.data.len() >> 1;
        let mut left_node_index = n + left;
        let mut right_node_index = n + right;
        let mut value_left = M::identity();
        let mut value_right = M::identity();
        while left_node_index < right_node_index {
            if left_node_index & 1 == 1 {
                value_left = M::operate(&value_left, &self.data[left_node_index]);
                left_node_index += 1;
            }
            if right_node_index & 1 == 1 {
                right_node_index -= 1;
                value_right = M::operate(&self.data[right_node_index], &value_right);
            }
            left_node_index >>= 1;
            right_node_index >>= 1;
        }
        M::operate(&value_left, &value_right)
    }

    pub fn max_right<F>(&self, is_ok: F, left: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(left <= self.size);
        if left == self.size {
            return self.size;
        }
        let n = self.data.len() >> 1;
        let mut value = M::identity();
        let mut node_index = (n + left) as i32;
        loop {
            node_index /= node_index & -node_index; // up to ceil
            if !is_ok(&M::operate(&value, &self.data[node_index as usize])) {
                break;
            }
            // up one stair from left
            value = M::operate(&value, &self.data[node_index as usize]);
            node_index += 1;
            if node_index & -node_index == node_index {
                // wall.
                return self.size;
            }
        }
        // down stairs to right
        while node_index < n as i32 {
            node_index <<= 1;
            if !is_ok(&M::operate(&value, &self.data[node_index as usize])) {
                continue;
            }
            value = M::operate(&value, &self.data[node_index as usize]);
            node_index += 1;
        }
        return node_index as usize - n;
    }

    pub fn min_left<F>(&self, is_ok: F, right: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(right <= self.size);
        if right == 0 {
            return 0;
        }
        let n = self.data.len() >> 1;
        let mut value = M::identity();
        let mut node_index = (n + right) as i32;
        loop {
            node_index /= node_index & -node_index;
            if !is_ok(&M::operate(&self.data[(node_index - 1) as usize], &value)) {
                break;
            }
            // up one stair from right
            node_index -= 1;
            value = M::operate(&self.data[node_index as usize], &value);
            if node_index & -node_index == node_index {
                // wall.
                return 0;
            }
        }
        while node_index < n as i32 {
            node_index <<= 1;
            if !is_ok(&M::operate(&self.data[(node_index - 1) as usize], &value)) {
                continue;
            }
            node_index -= 1;
            value = M::operate(&self.data[node_index as usize], &value);
        }
        return node_index as usize - n;
    }
}

impl<M: Monoid<S, T>, S, T> std::ops::Index<usize> for SegmentTree<M, S, T> {
    type Output = S;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size);
        &self.data[i + (self.data.len() >> 1)]
    }
}

pub struct SegmentTreeDFS<M: Monoid<S, T>, S = M, T = Additive> {
    phantom_t: std::marker::PhantomData<T>,
    phantom_m: std::marker::PhantomData<M>,
    size: usize,
    data: Vec<S>,
}

impl<M: Monoid<S, T>, S: Clone, T> From<&Vec<S>> for SegmentTreeDFS<M, S, T> {
    fn from(arr: &Vec<S>) -> Self {
        Self {
            phantom_t: std::marker::PhantomData,
            phantom_m: std::marker::PhantomData,
            size: arr.len(),
            data: make_data::<M, S, T>(arr),
        }
    }
}

impl<M: Monoid<S, T>, S, T> SegmentTreeDFS<M, S, T> {
    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        (&vec![M::identity(); size]).into()
    }

    pub fn set(&mut self, i: usize, x: S) {
        assert!(i < self.size);
        set::<M, S, T>(&mut self.data, i, x);
    }

    // pub fn get(&self, mut left: usize, mut right: usize) -> S {
    //     assert!(left <= right && right <= self.size);
    //     let n = self.data.len() >> 1;
    //     left += n;
    //     right += n;
    //     let mut value_left = M::identity();
    //     let mut value_right = M::identity();
    //     while left < right {
    //         if left & 1 == 1 {
    //             value_left = M::operate(&value_left,
    // &self.data[left]);             left += 1;
    //         }
    //         if right & 1 == 1 {
    //             right -= 1;
    //             value_right = M::operate(&self.data[right],
    // &value_right);         }
    //         left >>= 1;
    //         right >>= 1;
    //     }
    //     M::operate(&value_left, &value_right)
    // }

    // pub fn max_right<F>(&self, is_ok: F, left: usize) -> usize
    // where
    //     F: Fn(&S) -> bool,
    // {
    //     assert!(left < self.size);
    //     let n = self.data.len() >> 1;
    //     let mut value = M::identity();
    //     let mut node_index = (n + left) as i32;
    //     loop {
    //         node_index /= node_index & -node_index; // up to
    // ceil         if is_ok(&M::operate(&value,
    // &self.data[node_index as usize])) {             // up
    // one stair from left             value =
    // M::operate(&value, &self.data[node_index as usize]);
    //             node_index += 1;
    //             if node_index & -node_index == node_index {
    //                 // wall.
    //                 return self.size;
    //             }
    //             continue;
    //         }
    //         // down stairs to right
    //         while node_index < n as i32 {
    //             node_index <<= 1;
    //             if is_ok(&M::operate(&value,
    // &self.data[node_index as usize]))             {
    //                 value =
    //                     M::operate(&value, &self.data[node_index
    // as usize]);                 node_index += 1;
    //             }
    //         }
    //         return node_index as usize - n;
    //     }
    // }
}
impl<M: Monoid<S, T>, S, T> std::ops::Index<usize> for SegmentTreeDFS<M, S, T> {
    type Output = S;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size);
        &self.data[i + (self.data.len() >> 1)]
    }
}

#[cfg(test)]
mod tests {
    use crate::abstract_traits::{BinaryOperation, Identity};
    #[test]
    fn test_as_monoid() {
        impl BinaryOperation for usize {
            fn operate(x: &Self, y: &Self) -> Self { x + y }
        }
        impl Identity for usize {
            fn identity() -> Self { 0 }
        }

        let mut seg = super::SegmentTree::<usize>::new(10);
        assert_eq!(seg.get(0, 10), 0);
        seg.set(5, 5);
        assert_eq!(seg.get(0, 10), 5);
        seg.set(5, 10);
        assert_eq!(seg[5], 10);
        assert_eq!(seg[0], 0);
        let is_ok = |sum: &usize| *sum < 10;
        assert_eq!(seg.max_right(is_ok, 0), 5);
        assert_eq!(seg.max_right(is_ok, 5), 5);
        assert_eq!(seg.min_left(is_ok, 10), 6);
        assert_eq!(seg.min_left(is_ok, 5), 0);
        assert_eq!(seg.min_left(is_ok, 6), 6);
    }

    #[test]
    fn test_wrapping_monoid() {
        struct UsizeAdd;

        impl BinaryOperation<usize> for UsizeAdd {
            fn operate(x: &usize, y: &usize) -> usize { x + y }
        }
        impl Identity<usize> for UsizeAdd {
            fn identity() -> usize { 0 }
        }

        let mut seg = super::SegmentTree::<UsizeAdd, usize>::new(10);
        assert_eq!(seg.get(0, 10), 0);
        seg.set(0, 5);
        assert_eq!(seg.get(0, 10), 5);
        seg.set(0, 5);
        assert_eq!(seg[0], 5);
    }
}