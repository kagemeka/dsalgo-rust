use crate::static_matrix_property_trait::Shape;

pub trait Size {
    fn size() -> usize;
}

impl<P: Size> Shape for P {
    fn shape() -> (usize, usize) { (Self::size(), Self::size()) }
}
