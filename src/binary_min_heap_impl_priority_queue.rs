use crate::binary_min_heap_0_indexed::*;

type Q<T> = BinaryMinHeap<T>;

use crate::priority_queue_trait::{
    Pop,
    Push,
};

impl<T: Ord> Push for Q<T> {
    type T = T;

    fn push(
        &mut self,
        x: T,
    ) {
        Self::push(self, x);
    }
}

impl<T: Ord> Pop for Q<T> {
    type T = Option<T>;

    fn pop(&mut self) -> Self::T { Self::pop(self) }
}
