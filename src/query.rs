pub trait RangeGetQuery<I> {
    type T;

    fn get_range(
        &mut self,
        l: usize,
        r: usize,
    ) -> Self::T;
}

use crate::group_theory_id::Min;

pub trait RangeMinimumQuery {
    type T;

    fn range_minimum(
        &mut self,
        l: usize,
        r: usize,
    ) -> Self::T;
}

impl<T, Q> RangeMinimumQuery for Q
where
    Q: RangeGetQuery<Min, T = T>,
{
    type T = T;

    fn range_minimum(
        &mut self,
        l: usize,
        r: usize,
    ) -> Self::T {
        self.get_range(l, r)
    }
}
