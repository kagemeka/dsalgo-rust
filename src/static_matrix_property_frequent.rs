use crate::{
    matrix_with_static_property::Matrix,
    static_square_matrix_property_trait::Size,
};

#[derive(Debug, Clone)]

pub struct Prop100;

impl Size for Prop100 {
    fn size() -> usize { 100 }
}

pub type Mat100<T> = Matrix<T, Prop100>;

#[cfg(test)]

mod tests {

    #[test]

    fn test() {}
}
