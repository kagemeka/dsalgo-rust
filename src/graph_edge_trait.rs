pub trait Undirected {
    type V;

    fn u(&self) -> &Self::V;

    fn v(&self) -> &Self::V;
}

pub trait ToDirected {
    type E;

    fn to_directed(self) -> Self::E;
}

pub trait From {
    type V;

    fn from(&self) -> &Self::V;
}

pub trait To {
    type V;

    fn to(&self) -> &Self::V;
}

pub trait Reverse {
    fn reverse(self) -> Self;
}

pub trait Value {
    type T;

    fn value(&mut self) -> &Self::T;
}

pub trait ValueMut {
    type T;

    fn value_mut(&mut self) -> &mut Self::T;
}

pub trait Weight {
    type T;

    fn weight(&self) -> &Self::T;
}

pub trait WeightMut: Weight {
    fn weight_mut(&mut self) -> &mut Self::T;
}

pub trait Capacity<T> {
    fn capacity(&self) -> &T;
}
