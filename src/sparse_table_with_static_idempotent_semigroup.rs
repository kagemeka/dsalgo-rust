//! sparse table

use std::iter::FromIterator;

use crate::{
    algebraic_structure::*,
    binary_function::*,
};

pub struct SparseTable<G: Semigroup> {
    node: Vec<Vec<G::S>>,
}

impl<G> std::iter::FromIterator<G::S> for SparseTable<G>
where
    G: Semigroup + Idempotence + Commutative,
    G::S: Clone,
{
    fn from_iter<T: IntoIterator<Item = G::S>>(iter: T) -> Self {
        let mut node = vec![iter.into_iter().collect::<Vec<_>>()];

        let max_width = node[0].len();

        assert!(max_width > 0);

        let height = if max_width <= 1 {
            1
        } else {
            max_width.next_power_of_two().trailing_zeros() as usize
        };

        for i in 1..height {
            let row_size = max_width - (1 << i) + 1;

            // last is max_width - (1 << i) covering (1 << i)
            // including the position.
            node.push(
                (0..row_size)
                    .map(|j| {
                        G::op(
                            node[i - 1][j].clone(),
                            node[i - 1][j + (1 << (i - 1))].clone(),
                        )
                    })
                    .collect(),
            );
        }

        Self { node }
    }
}

impl<G> SparseTable<G>
where
    G: Semigroup + Idempotence + Commutative,
    G::S: Clone,
{
    pub fn new(slice: &[G::S]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }

    pub fn size(&self) -> usize { self.node[0].len() }

    pub fn reduce(
        &self,
        l: usize,
        r: usize,
    ) -> G::S {
        assert!(l < r && r <= self.size());

        if r - l == 1 {
            return self.node[0][l].clone();
        }

        let i = (r - l).next_power_of_two().trailing_zeros() as usize - 1;

        G::op(self.node[i][l].clone(), self.node[i][r - (1 << i)].clone())
    }
}

use crate::{
    algebraic_structure_impl::*,
    query::RangeGetQuery,
};

impl<S, I> RangeGetQuery<I> for SparseTable<GroupApprox<S, I>>
where
    GroupApprox<S, I>: Semigroup<S = S> + Idempotence + Commutative,
    S: Clone,
{
    type T = S;

    fn get_range(
        &mut self,
        l: usize,
        r: usize,
    ) -> Self::T {
        self.reduce(l, r)
    }
}

use crate::bit_length_with_count_leading_zeros_u64::bit_length;

pub struct DisjointSparseTable<G: Semigroup> {
    node: Vec<Vec<G::S>>,
}

impl<G> std::iter::FromIterator<G::S> for DisjointSparseTable<G>
where
    G: Semigroup + Commutative,
    G::S: Clone,
{
    fn from_iter<T: IntoIterator<Item = G::S>>(iter: T) -> Self {
        let mut node = vec![iter.into_iter().collect::<Vec<_>>()];

        let size = node[0].len();

        let height = if size <= 1 {
            1
        } else {
            size.next_power_of_two().trailing_zeros() as usize
        };

        for i in 1..height {
            let mut row = node[0].clone();

            for p in (1 << i..=size).step_by(2 << i) {
                for d in 1..(1 << i) {
                    let j = p - d;

                    row[j - 1] = G::op(row[j - 1].clone(), row[j].clone());
                }

                for d in 0..(1 << i) - 1 {
                    let j = p + d;

                    if j + 1 >= size {
                        break;
                    }

                    row[j + 1] = G::op(row[j].clone(), row[j + 1].clone());
                }
            }

            node.push(row);
        }

        Self { node }
    }
}

impl<G> DisjointSparseTable<G>
where
    G: Semigroup + Commutative,
    G::S: Clone,
{
    pub fn new(slice: &[G::S]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }

    pub fn size(&self) -> usize { self.node[0].len() }

    /// [l, r)

    pub fn reduce(
        &self,
        l: usize,
        mut r: usize,
    ) -> G::S {
        assert!(l < r && r <= self.size());

        r -= 1; // internally, consider [l, r]
        if l == r {
            return self.node[0][l].clone();
        }

        let i = bit_length((l ^ r) as u64) as usize - 1;

        // if i = 0, then use 0-th row.
        // if i = 3, then use 3-th row.
        // what does this mean?
        // only msb of l \xor r is important.
        // because,
        // for each bit j (checking in descending order from top bit),
        // if for any k in 2^j..=|node| (step 2^{j + 1}), l < k <= r,
        // then j-th bit of l \xor r is gonna be 1.
        // so the query can be dealed with j-th row.
        // <->
        // if j-th bit of l \xor r is 0,
        // then for all k in 2^j..=|node| (step 2^{j + 1}),
        // k <= l < r or l < r < k.
        // so the query cannot be dealed with j-th row.
        // then, check {j-1}-th bit next...
        G::op(self.node[i][l].clone(), self.node[i][r].clone())
    }
}

impl<S, I> RangeGetQuery<I> for DisjointSparseTable<GroupApprox<S, I>>
where
    GroupApprox<S, I>: Semigroup<S = S> + Commutative,
    S: Clone,
{
    type T = S;

    fn get_range(
        &mut self,
        l: usize,
        r: usize,
    ) -> Self::T {
        self.reduce(l, r)
    }
}

// TODO:
// reference
// https://github.com/maspypy/library/blob/main/ds/xor_sparsetable.hpp
pub mod xor_sparse_table {}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_sparse_table() {
        use crate::group_theory_id::Min;

        let arr: Vec<usize> = vec![0, 4, 2, 8, 5, 1];

        let sp = SparseTable::<GroupApprox<usize, Min>>::new(&arr);

        assert_eq!(sp.reduce(0, 4), 0);

        assert_eq!(sp.reduce(3, 4), 8);

        assert_eq!(sp.reduce(1, 6), 1);
    }

    #[test]

    fn test_disjoint_sparse_table() {
        use crate::group_theory_id::Min;

        let arr: Vec<usize> = vec![0, 4, 2, 8, 5, 1];

        let sp = DisjointSparseTable::<GroupApprox<usize, Min>>::new(&arr);

        assert_eq!(sp.reduce(0, 4), 0);

        assert_eq!(sp.reduce(3, 4), 8);

        assert_eq!(sp.reduce(1, 6), 1);
    }
}
