use std::cmp::Reverse;

type Q<T> = std::collections::BinaryHeap<Reverse<T>>;

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
        Self::push(self, Reverse(x));
    }
}

impl<T: Ord> Pop for Q<T> {
    type T = Option<T>;

    fn pop(&mut self) -> Self::T {
        if let Some(Reverse(x)) = Self::pop(self) {
            Some(x)
        } else {
            None
        }
    }
}
