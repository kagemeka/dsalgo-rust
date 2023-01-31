use crate::graph::edge::{
    From,
    Reversed,
    To,
    ToDirected,
    Value,
    Weight,
};

impl<T> ToDirected for (usize, usize, T) {
    type E = Self;

    fn to_directed(self) -> Self::E { self }
}

impl<T> Reversed for (usize, usize, T) {
    fn reversed(mut self) -> Self {
        std::mem::swap(&mut self.0, &mut self.1);

        self
    }
}

impl<T> From for (usize, usize, T) {
    type V = usize;

    fn from(&self) -> &Self::V { &self.0 }
}

impl<T> To for (usize, usize, T) {
    type V = usize;

    fn to(&self) -> &Self::V { &self.1 }
}

impl<T> Value for (usize, usize, T) {
    type T = T;

    fn value(&self) -> &Self::T { &self.2 }
}

impl<T: Weight<U>, U> Weight<U> for (usize, usize, T) {
    fn weight(&self) -> &U { &self.2.weight() }
}

// impled U: see graph_edge_weight_impl
impl Weight<Self> for u64 {
    fn weight(&self) -> &Self { self }
}

impl Weight<Self> for i64 {
    fn weight(&self) -> &Self { self }
}
