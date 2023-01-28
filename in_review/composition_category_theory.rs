use crate::category::Category;

pub trait Composition: Category {
    fn compose(
        f: Self::Morphism,
        g: Self::Morphism,
    ) -> Self::Morphism;
}
