use crate::choose::Choose;

pub struct HomogeneousProduct<T> {
    chooser: Box<dyn Choose<T>>,
}

impl<T> HomogeneousProduct<T> {
    pub fn new(chooser: Box<dyn Choose<T>>) -> Self { Self { chooser } }

    pub fn calc(
        &mut self,
        n: u64,
        k: u64,
    ) -> T
    where
        T: From<u64>,
    {
        if n == 0 {
            0.into()
        } else {
            self.chooser.choose(n + k - 1, k)
        }
    }
}

#[cfg(test)]

mod tests {

    #[test]

    fn test() {
        use super::HomogeneousProduct;
        use crate::{
            combination_from_u64::Combination,
            default_static_modular_arithmetic::Modular1_000_000_007,
            modular_int_with_arithmetic::Modint,
        };

        type Mint = Modint<u32, Modular1_000_000_007>;

        let mut hom =
            HomogeneousProduct::<Mint>::new(Box::new(
                Combination::<Mint>::new(100),
            ));

        assert_eq!(hom.calc(5, 2).value(), 15);
    }
}
